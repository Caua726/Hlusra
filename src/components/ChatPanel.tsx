import { useState, useEffect, useRef, useCallback } from "react";
import { listen } from "@tauri-apps/api/event";
import { chatMessage, indexMeeting, getChatStatus } from "../lib/api";
import type { ChatStatus } from "../lib/api";
import { formatError } from "../lib/format";

interface Props {
  meetingId: string;
  chatStatus: ChatStatus;
  meetingTitle: string;
  onBack: () => void;
  onStatusChange: (status: ChatStatus) => void;
}

interface ChatMsg {
  role: "user" | "assistant";
  content: string;
}

export default function ChatPanel({ meetingId, chatStatus, meetingTitle, onBack, onStatusChange }: Props) {
  const [messages, setMessages] = useState<ChatMsg[]>([]);
  const [input, setInput] = useState("");
  const [sending, setSending] = useState(false);
  const [indexing, setIndexing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [currentStatus, setCurrentStatus] = useState<ChatStatus>(chatStatus);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const mountedRef = useRef(true);
  const sendingRef = useRef(false);
  const cleanupListenersRef = useRef<(() => void) | null>(null);

  useEffect(() => {
    mountedRef.current = true;
    return () => {
      mountedRef.current = false;
      cleanupListenersRef.current?.();
    };
  }, []);

  useEffect(() => {
    setCurrentStatus(chatStatus);
  }, [chatStatus]);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const appendToLastAssistant = useCallback((text: string) => {
    setMessages((prev) => {
      const updated = [...prev];
      const last = updated[updated.length - 1];
      if (last && last.role === "assistant") {
        updated[updated.length - 1] = { ...last, content: last.content + text };
      }
      return updated;
    });
  }, []);

  async function handleIndex() {
    setError(null);
    setIndexing(true);
    try {
      await indexMeeting(meetingId);
      const status = await getChatStatus(meetingId);
      if (!mountedRef.current) return;
      setCurrentStatus(status);
      onStatusChange(status);
    } catch (e) {
      if (!mountedRef.current) return;
      setError(formatError(e));
    } finally {
      if (mountedRef.current) setIndexing(false);
    }
  }

  async function handleSend() {
    const msg = input.trim();
    if (!msg || sending || sendingRef.current) return;

    sendingRef.current = true;
    setError(null);
    setInput("");
    setSending(true);

    setMessages((prev) => [
      ...prev,
      { role: "user", content: msg },
      { role: "assistant", content: "" },
    ]);

    const unlistenChunk = await listen<string>("chat-stream-chunk", (event) => {
      appendToLastAssistant(event.payload);
    });

    let resolveDone: () => void;
    const streamFinished = new Promise<void>((resolve) => {
      resolveDone = resolve;
    });

    const unlistenDone = await listen<void>("chat-stream-done", () => {
      resolveDone();
    });

    const unlistenError = await listen<string>("chat-stream-error", (event) => {
      if (mountedRef.current) setError(event.payload);
      resolveDone();
    });

    const cleanup = () => {
      unlistenChunk();
      unlistenDone();
      unlistenError();
      cleanupListenersRef.current = null;
    };
    cleanupListenersRef.current = cleanup;

    try {
      await chatMessage(meetingId, msg);
      await streamFinished;
    } catch (e) {
      if (mountedRef.current) setError(formatError(e));
    } finally {
      cleanup();
      if (mountedRef.current) setSending(false);
      sendingRef.current = false;
    }
  }

  function handleKeyDown(e: React.KeyboardEvent) {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  }

  // Not indexed or failed - show index prompt
  if (currentStatus === "not_indexed" || currentStatus === "failed") {
    return (
      <>
        <header className="glass shrink-0 border-b border-white/5">
          <div className="px-5 h-12 flex items-center gap-3">
            <button onClick={onBack} aria-label="Voltar" className="text-white/25 hover:text-white/60 transition-colors p-1.5 rounded-lg hover:bg-white/5 border-0 bg-transparent cursor-pointer">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M15 19l-7-7 7-7" />
              </svg>
            </button>
            <h1 className="text-sm font-semibold text-white/80">Chat</h1>
            <span className="text-[10px] text-white/20">{meetingTitle}</span>
          </div>
        </header>
        <div className="flex-1 flex flex-col items-center justify-center gap-4 p-8">
          {currentStatus === "failed" && (
            <p className="text-red-400/80 text-xs" role="alert">A indexação falhou.</p>
          )}
          <p className="text-white/30 text-[13px]">A reunião precisa ser indexada antes de usar o chat.</p>
          <button
            className="px-6 py-2.5 bg-brand-500 hover:bg-brand-600 text-white rounded-xl text-[12px] font-medium transition-all active:scale-[0.98] glow-sm cursor-pointer border-0 disabled:opacity-40"
            onClick={handleIndex}
            disabled={indexing}
          >
            {indexing ? "Indexando..." : "Indexar reunião"}
          </button>
          {error && <p className="text-red-400/80 text-xs" role="alert">{error}</p>}
        </div>
      </>
    );
  }

  // Indexing
  if (currentStatus === "indexing" || indexing) {
    return (
      <>
        <header className="glass shrink-0 border-b border-white/5">
          <div className="px-5 h-12 flex items-center gap-3">
            <button onClick={onBack} aria-label="Voltar" className="text-white/25 hover:text-white/60 transition-colors p-1.5 rounded-lg hover:bg-white/5 border-0 bg-transparent cursor-pointer">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M15 19l-7-7 7-7" />
              </svg>
            </button>
            <h1 className="text-sm font-semibold text-white/80">Chat</h1>
            <span className="text-[10px] text-white/20">{meetingTitle}</span>
          </div>
        </header>
        <div className="flex-1 flex flex-col items-center justify-center gap-3 text-white/30">
          <div className="w-6 h-6 border-[3px] border-white/10 border-t-brand-500 rounded-full animate-[spin_0.7s_linear_infinite]" />
          <p className="text-[13px]">Indexando reunião...</p>
        </div>
      </>
    );
  }

  // Ready - full chat view
  return (
    <>
      <header className="glass shrink-0 border-b border-white/5">
        <div className="px-5 h-12 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <button onClick={onBack} aria-label="Voltar" className="text-white/25 hover:text-white/60 transition-colors p-1.5 rounded-lg hover:bg-white/5 border-0 bg-transparent cursor-pointer">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M15 19l-7-7 7-7" />
              </svg>
            </button>
            <h1 className="text-sm font-semibold text-white/80">Chat</h1>
            <span className="text-[10px] text-white/20">{meetingTitle}</span>
          </div>
          <div className="flex items-center gap-1.5">
            <div className="w-1.5 h-1.5 rounded-full bg-emerald-500/70 animate-pulse" />
            <span className="text-[10px] text-white/30">Pronto</span>
          </div>
        </div>
      </header>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto p-5 space-y-4">
        {messages.length === 0 && (
          <div className="flex items-center justify-center h-full">
            <p className="text-white/20 text-[13px]">Faça uma pergunta sobre esta reunião.</p>
          </div>
        )}
        {messages.map((msg, i) => (
          <div key={i} className={`flex ${msg.role === "user" ? "justify-end" : "justify-start"}`}>
            <div className={`rounded-2xl px-4 py-3 ${msg.role === "user" ? "max-w-[75%]" : "max-w-[80%]"} ${
              msg.role === "user"
                ? "bg-brand-500/8 border border-brand-500/10 rounded-br-md"
                : "glass-heavy rounded-bl-md"
            }`}>
              <p className={`text-[13px] leading-relaxed break-words whitespace-pre-wrap ${
                msg.role === "user" ? "text-white/70" : "text-white/50"
              }`}>
                {msg.content || (sending && msg.role === "assistant" ? "..." : "")}
              </p>
            </div>
          </div>
        ))}
        <div ref={messagesEndRef} />
      </div>

      {/* Input */}
      <div className="p-4 border-t border-white/5 shrink-0">
        <div className="flex gap-2 items-end">
          <textarea
            rows={2}
            placeholder="Pergunte sobre a reunião..."
            value={input}
            onChange={(e) => {
              setInput(e.target.value);
              // Auto-grow: reset height then set to scrollHeight
              e.target.style.height = "auto";
              e.target.style.height = `${Math.min(e.target.scrollHeight, 120)}px`;
            }}
            onKeyDown={handleKeyDown}
            disabled={sending}
            className="flex-1 glass-input rounded-xl px-4 py-3 text-[13px] text-white/70 placeholder-white/20 focus:outline-none focus:border-brand-500/30 focus:shadow-[0_0_0_3px_rgba(244,63,94,0.08)] transition-all resize-none"
          />
          <button
            onClick={handleSend}
            disabled={sending || !input.trim()}
            aria-label="Enviar mensagem"
            className="px-5 py-3 bg-gradient-to-r from-brand-500 to-brand-600 text-white rounded-xl text-[12px] font-medium transition-all active:scale-95 glow-sm hover:shadow-lg hover:shadow-brand-500/20 border-0 cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
            </svg>
          </button>
        </div>
        {error && <p className="text-red-400/80 text-xs mt-2" role="alert">{error}</p>}
      </div>
    </>
  );
}
