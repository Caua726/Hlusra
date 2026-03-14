import { useState, useEffect, useRef, useCallback } from "react";
import { listen } from "@tauri-apps/api/event";
import { chatMessage, indexMeeting, getChatStatus } from "../lib/api";

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
      setCurrentStatus(status as "not_indexed" | "indexing" | "ready" | "failed");
      onStatusChange();
    } catch (e) {
      setError(String(e));
    } finally {
      setIndexing(false);
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
      setError(event.payload);
      resolveDone();
    });

    try {
      await chatMessage(meetingId, msg);
      await streamFinished;
    } catch (e) {
      setError(String(e));
    } finally {
      unlistenChunk();
      unlistenDone();
      unlistenError();
      setSending(false);
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
      <div className="chat-panel">
        <h3>Chat</h3>
        <div className="chat-index-prompt">
          {currentStatus === "failed" && (
            <p className="error-text">A indexacao falhou.</p>
          )}
          <p>A reuniao precisa ser indexada antes de usar o chat.</p>
          <button className="btn-primary" onClick={handleIndex} disabled={indexing}>
            {indexing ? "Indexando..." : "Indexar reuniao"}
          </button>
          {error && <p className="error-text">{error}</p>}
        </div>
      </div>
    );
  }

  if (currentStatus === "indexing" || indexing) {
    return (
      <div className="chat-panel">
        <h3>Chat</h3>
        <div className="chat-index-prompt">
          <div className="spinner" />
          <p>Indexando reuniao...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="chat-panel">
      <h3>Chat</h3>
      <div className="chat-messages">
        {messages.length === 0 && (
          <p className="chat-empty">Faca uma pergunta sobre esta reuniao.</p>
        )}
        {messages.map((msg, i) => (
          <div key={i} className={`chat-bubble chat-${msg.role}`}>
            <p>{msg.content || (sending && msg.role === "assistant" ? "..." : "")}</p>
          </div>
        ))}
        <div ref={messagesEndRef} />
      </div>
      <div className="chat-input-row">
        <textarea
          className="chat-input"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Digite sua pergunta..."
          disabled={sending}
          rows={1}
        />
        <button
          className="btn-primary chat-send"
          onClick={handleSend}
          disabled={sending || !input.trim()}
        >
          Enviar
        </button>
      </div>
      {error && <p className="error-text">{error}</p>}
    </div>
  );
}
