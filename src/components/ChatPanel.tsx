import { useState, useEffect, useRef, useCallback } from "react";
import { listen } from "@tauri-apps/api/event";
import { chatMessage, indexMeeting, getChatStatus } from "../lib/api";
import { formatError } from "../lib/format";

interface Props {
  meetingId: string;
  chatStatus: "not_indexed" | "indexing" | "ready" | "failed";
  onStatusChange: () => void;
}

interface ChatMsg {
  role: "user" | "assistant";
  content: string;
}

export default function ChatPanel({ meetingId, chatStatus, onStatusChange }: Props) {
  const [messages, setMessages] = useState<ChatMsg[]>([]);
  const [input, setInput] = useState("");
  const [sending, setSending] = useState(false);
  const [indexing, setIndexing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [currentStatus, setCurrentStatus] = useState(chatStatus);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const streamBufferRef = useRef("");
  const mountedRef = useRef(true);
  const cleanupListenersRef = useRef<(() => void) | null>(null);

  useEffect(() => {
    mountedRef.current = true;
    return () => {
      mountedRef.current = false;
      // Clean up any active listeners on unmount
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
      setCurrentStatus(status as "not_indexed" | "indexing" | "ready" | "failed");
      onStatusChange();
    } catch (e) {
      if (!mountedRef.current) return;
      setError(formatError(e));
    } finally {
      if (mountedRef.current) setIndexing(false);
    }
  }

  async function handleSend() {
    const msg = input.trim();
    if (!msg || sending) return;

    setError(null);
    setInput("");
    setSending(true);
    streamBufferRef.current = "";

    setMessages((prev) => [
      ...prev,
      { role: "user", content: msg },
      { role: "assistant", content: "" },
    ]);

    // Await all listeners BEFORE calling chatMessage to avoid race conditions
    const unlistenChunk = await listen<string>("chat-stream-chunk", (event) => {
      streamBufferRef.current += event.payload;
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
    }
  }

  function handleKeyDown(e: React.KeyboardEvent) {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  }

  if (currentStatus === "not_indexed" || currentStatus === "failed") {
    return (
      <div className="mb-8">
        <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-zinc-700">Chat</h3>
        <div className="flex flex-col items-center gap-3 p-8 text-center text-zinc-500">
          {currentStatus === "failed" && (
            <p className="text-red-500 text-sm">A indexacao falhou.</p>
          )}
          <p>A reuniao precisa ser indexada antes de usar o chat.</p>
          <button className="bg-rose-500 text-white border-none px-6 py-2.5 rounded-lg text-sm cursor-pointer transition-colors duration-150 font-medium hover:bg-rose-600 disabled:opacity-40 disabled:cursor-not-allowed" onClick={handleIndex} disabled={indexing}>
            {indexing ? "Indexando..." : "Indexar reuniao"}
          </button>
          {error && <p className="text-red-500 text-sm">{error}</p>}
        </div>
      </div>
    );
  }

  if (currentStatus === "indexing" || indexing) {
    return (
      <div className="mb-8">
        <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-zinc-700">Chat</h3>
        <div className="flex flex-col items-center gap-3 p-8 text-center text-zinc-500">
          <div className="w-6 h-6 border-[3px] border-zinc-700 border-t-rose-500 rounded-full animate-[spin_0.7s_linear_infinite]" />
          <p>Indexando reuniao...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="mb-8">
      <h3 className="text-lg font-semibold mb-4 pb-2 border-b border-zinc-700">Chat</h3>
      <div className="max-h-[400px] overflow-y-auto py-2 mb-4 flex flex-col gap-3">
        {messages.length === 0 && (
          <p className="text-center text-zinc-600 py-8 text-sm">Faca uma pergunta sobre esta reuniao.</p>
        )}
        {messages.map((msg, i) => (
          <div
            key={i}
            className={`max-w-[80%] px-4 py-3 rounded-xl text-sm leading-normal break-words whitespace-pre-wrap ${
              msg.role === "user"
                ? "self-end bg-rose-500 text-white rounded-br-sm"
                : "self-start bg-zinc-800/50 text-zinc-100 border border-zinc-700 rounded-bl-sm"
            }`}
          >
            <p>{msg.content || (sending && msg.role === "assistant" ? "..." : "")}</p>
          </div>
        ))}
        <div ref={messagesEndRef} />
      </div>
      <div className="flex gap-2 items-end">
        <textarea
          className="flex-1 bg-zinc-800 text-zinc-100 border border-zinc-700 rounded-lg px-3 py-2.5 text-sm font-[inherit] resize-none outline-none transition-colors duration-150 min-h-[40px] focus:border-blue-500"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Digite sua pergunta..."
          disabled={sending}
          rows={1}
        />
        <button
          className="bg-rose-500 text-white border-none px-6 py-2.5 rounded-lg text-sm cursor-pointer transition-colors duration-150 font-medium hover:bg-rose-600 disabled:opacity-40 disabled:cursor-not-allowed shrink-0"
          onClick={handleSend}
          disabled={sending || !input.trim()}
        >
          Enviar
        </button>
      </div>
      {error && <p className="text-red-500 text-sm mt-2">{error}</p>}
    </div>
  );
}
