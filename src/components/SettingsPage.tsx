import { useState, useEffect, useRef, useCallback } from "react";
import {
  getSettings,
  updateSettings,
  probeEncoders,
  listAvailableModels,
  downloadModel,
  getActiveModel,
  setActiveModel,
} from "../lib/api";
import type { AppSettings, WhisperModel } from "../lib/api";
import { formatError } from "../lib/format";

interface Props {
  onBack: () => void;
}

type SettingsTab = "geral" | "video" | "audio" | "trans" | "rag";

const TABS: { id: SettingsTab; label: string }[] = [
  { id: "geral", label: "Geral" },
  { id: "video", label: "Vídeo" },
  { id: "audio", label: "Áudio" },
  { id: "trans", label: "Transcrição" },
  { id: "rag", label: "RAG / Chat" },
];

export default function SettingsPage({ onBack }: Props) {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [saved, setSaved] = useState(false);
  const [activeTab, setActiveTab] = useState<SettingsTab>("geral");
  const savedTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const [isDirty, setIsDirty] = useState(false);

  // Encoder probing
  const [encoders, setEncoders] = useState<Record<string, string[]>>({});

  // Whisper models
  const [models, setModels] = useState<WhisperModel[]>([]);
  const [activeModelName, setActiveModelName] = useState("");
  const [downloadingModel, setDownloadingModel] = useState<string | null>(null);

  useEffect(() => {
    loadAll();
    return () => {
      if (savedTimerRef.current) clearTimeout(savedTimerRef.current);
    };
  }, []);

  async function loadAll() {
    try {
      const [s, enc, mods, active] = await Promise.all([
        getSettings(),
        probeEncoders().catch(() => ({} as Record<string, string[]>)),
        listAvailableModels().catch(() => [] as WhisperModel[]),
        getActiveModel().catch(() => ({ name: "base" } as WhisperModel)),
      ]);
      setSettings(s);
      setEncoders(enc);
      setModels(mods);
      setActiveModelName(active.name);
    } catch (e) {
      setError(formatError(e));
    } finally {
      setLoading(false);
    }
  }

  function update(fn: (s: AppSettings) => AppSettings) {
    setSaved(false);
    setIsDirty(true);
    setSettings((prev) => prev ? fn(prev) : prev);
  }

  const handleBack = useCallback(() => {
    if (isDirty) {
      if (!window.confirm("Existem alterações não salvas. Deseja sair mesmo assim?")) {
        return;
      }
    }
    onBack();
  }, [isDirty, onBack]);

  function handleTabSwitch(tab: SettingsTab) {
    setActiveTab(tab);
    setError(null);
    setSaved(false);
  }

  async function handleSave() {
    if (!settings) return;
    setError(null);
    setSaving(true);
    setSaved(false);
    try {
      await updateSettings(settings);
      setSaved(true);
      setIsDirty(false);
      if (savedTimerRef.current) clearTimeout(savedTimerRef.current);
      savedTimerRef.current = setTimeout(() => setSaved(false), 3000);
    } catch (e) {
      setError(formatError(e));
    } finally {
      setSaving(false);
    }
  }

  async function handleDownloadModel(name: string) {
    setDownloadingModel(name);
    try {
      await downloadModel(name);
      const mods = await listAvailableModels();
      setModels(mods);
    } catch (e) {
      setError(formatError(e));
    } finally {
      setDownloadingModel(null);
    }
  }

  async function handleSetActiveModel(name: string) {
    try {
      await setActiveModel(name);
      setActiveModelName(name);
    } catch (e) {
      setError(formatError(e));
    }
  }

  const inputCls = "w-full glass-input rounded-xl px-4 py-2.5 text-[12px] text-white/70 focus:outline-none focus:border-white/20 transition-all";
  const selectCls = "w-full glass-input rounded-xl px-4 py-2.5 text-[12px] text-white/70 appearance-none cursor-pointer focus:outline-none transition-all";

  if (loading) {
    return (
      <>
        <header className="glass shrink-0 border-b border-white/5">
          <div className="px-5 h-12 flex items-center gap-3">
            <button onClick={handleBack} aria-label="Voltar" className="text-white/25 hover:text-white/60 transition-colors p-1.5 rounded-lg hover:bg-white/5 border-0 bg-transparent cursor-pointer">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M15 19l-7-7 7-7" />
              </svg>
            </button>
            <h1 className="text-sm font-semibold text-white/80">Configurações</h1>
          </div>
        </header>
        <div className="flex-1 flex items-center justify-center">
          <div className="w-6 h-6 border-[3px] border-white/10 border-t-brand-500 rounded-full animate-[spin_0.7s_linear_infinite]" />
        </div>
      </>
    );
  }

  if (!settings) {
    return (
      <>
        <header className="glass shrink-0 border-b border-white/5">
          <div className="px-5 h-12 flex items-center gap-3">
            <button onClick={handleBack} aria-label="Voltar" className="text-white/25 hover:text-white/60 transition-colors p-1.5 rounded-lg hover:bg-white/5 border-0 bg-transparent cursor-pointer">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M15 19l-7-7 7-7" />
              </svg>
            </button>
            <h1 className="text-sm font-semibold text-white/80">Configurações</h1>
          </div>
        </header>
        <div className="flex-1 flex items-center justify-center">
          <p className="text-red-400/80 text-xs">{error || "Falha ao carregar configurações."}</p>
        </div>
      </>
    );
  }

  const backendOptions = Object.keys(encoders);

  return (
    <>
      {/* Header */}
      <header className="glass shrink-0 border-b border-white/5">
        <div className="px-5 h-12 flex items-center gap-3">
          <button onClick={handleBack} aria-label="Voltar" className="text-white/25 hover:text-white/60 transition-colors p-1.5 rounded-lg hover:bg-white/5 border-0 bg-transparent cursor-pointer">
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M15 19l-7-7 7-7" />
            </svg>
          </button>
          <h1 className="text-sm font-semibold text-white/80">Configurações</h1>
        </div>
      </header>

      <div className="flex-1 flex overflow-hidden">
        {/* Sidebar */}
        <nav className="w-44 shrink-0 border-r border-white/5 py-3 px-2 space-y-0.5">
          {TABS.map((tab) => (
            <button
              key={tab.id}
              onClick={() => handleTabSwitch(tab.id)}
              className={`w-full text-left px-3 py-2 rounded-lg text-[12px] transition-all border-0 cursor-pointer bg-transparent ${
                activeTab === tab.id
                  ? "text-white/70 bg-white/5 font-medium"
                  : "text-white/30 hover:text-white/60 hover:bg-white/[0.03]"
              }`}
            >
              {tab.label}
            </button>
          ))}
        </nav>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-6">
          {/* Geral */}
          {activeTab === "geral" && (
            <div className="space-y-5">
              <h2 className="text-[10px] font-bold text-white/25 uppercase tracking-[0.2em] mb-4">Geral</h2>
              <div>
                <label className="block text-[12px] text-white/50 mb-2">Diretório de gravações</label>
                <input
                  type="text"
                  className={inputCls}
                  value={settings.general.recordings_dir}
                  onChange={(e) =>
                    update((s) => ({
                      ...s,
                      general: { ...s.general, recordings_dir: e.target.value },
                    }))
                  }
                />
              </div>
              <div>
                <label className="block text-[12px] text-white/50 mb-2">Nome automático</label>
                <input
                  type="text"
                  className={inputCls}
                  value={settings.general.auto_meeting_name}
                  onChange={(e) =>
                    update((s) => ({
                      ...s,
                      general: { ...s.general, auto_meeting_name: e.target.value },
                    }))
                  }
                />
                <p className="text-[10px] text-white/15 mt-1.5">Variáveis disponíveis: {"{date}"}, {"{time}"}</p>
              </div>
              <div className="flex items-center justify-between py-2">
                <div>
                  <label className="text-[12px] text-white/50">Iniciar minimizado</label>
                  <p className="text-[10px] text-white/15 mt-0.5">Abre na bandeja do sistema</p>
                </div>
                <label className="relative cursor-pointer">
                  <input
                    type="checkbox"
                    className="sr-only peer"
                    checked={settings.general.start_minimized}
                    onChange={(e) =>
                      update((s) => ({
                        ...s,
                        general: { ...s.general, start_minimized: e.target.checked },
                      }))
                    }
                  />
                  <div className="w-10 h-[22px] rounded-full bg-white/5 border border-white/10 peer-checked:bg-brand-500/15 peer-checked:border-brand-500/40 transition-all" />
                  <div className="absolute left-[3px] top-[3px] w-4 h-4 rounded-full bg-white/20 peer-checked:bg-brand-500 peer-checked:translate-x-[18px] transition-all pointer-events-none peer-checked:shadow-[0_0_12px_rgba(244,63,94,0.6)]" />
                </label>
              </div>
              <div className="pt-4 border-t border-white/5 flex items-center gap-3">
                <button
                  className="px-6 py-2.5 bg-brand-500 hover:bg-brand-600 text-white rounded-xl text-[12px] font-medium transition-all active:scale-[0.98] glow-sm border-0 cursor-pointer disabled:opacity-40"
                  onClick={handleSave}
                  disabled={saving}
                >
                  {saving ? "Salvando..." : "Salvar"}
                </button>
                {saved && <span className="text-emerald-400 text-[11px]">Salvo!</span>}
                {error && <span className="text-red-400/80 text-xs">{error}</span>}
              </div>
            </div>
          )}

          {/* Video */}
          {activeTab === "video" && (
            <div className="space-y-5">
              <h2 className="text-[10px] font-bold text-white/25 uppercase tracking-[0.2em] mb-4">Vídeo</h2>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-[12px] text-white/50 mb-2">Codec</label>
                  <select
                    className={selectCls}
                    value={settings.video.codec}
                    onChange={(e) =>
                      update((s) => ({
                        ...s,
                        video: { ...s.video, codec: e.target.value },
                      }))
                    }
                  >
                    <option value="h265">H.265</option>
                    <option value="h264">H.264</option>
                    <option value="av1">AV1</option>
                  </select>
                </div>
                <div>
                  <label className="block text-[12px] text-white/50 mb-2">Backend</label>
                  <select
                    className={selectCls}
                    value={settings.video.backend}
                    onChange={(e) =>
                      update((s) => ({
                        ...s,
                        video: { ...s.video, backend: e.target.value },
                      }))
                    }
                  >
                    {backendOptions.length > 0 ? (
                      backendOptions.map((b) => (
                        <option key={b} value={b}>
                          {b.charAt(0).toUpperCase() + b.slice(1)}
                        </option>
                      ))
                    ) : (
                      <>
                        <option value="vaapi">Vaapi</option>
                        <option value="cuda">Cuda</option>
                        <option value="vulkan">Vulkan</option>
                        <option value="software">Software</option>
                      </>
                    )}
                  </select>
                </div>
                <div>
                  <label className="block text-[12px] text-white/50 mb-2">FPS</label>
                  <input
                    type="number"
                    className={inputCls}
                    value={settings.video.fps}
                    onChange={(e) =>
                      update((s) => ({
                        ...s,
                        video: { ...s.video, fps: Number(e.target.value) || s.video.fps },
                      }))
                    }
                  />
                </div>
                <div>
                  <label className="block text-[12px] text-white/50 mb-2">Resolução</label>
                  <select
                    className={selectCls}
                    value={settings.video.resolution}
                    onChange={(e) =>
                      update((s) => ({
                        ...s,
                        video: { ...s.video, resolution: e.target.value },
                      }))
                    }
                  >
                    <option value="480p">480p</option>
                    <option value="720p">720p</option>
                    <option value="1080p">1080p</option>
                    <option value="1440p">1440p</option>
                    <option value="2160p">2160p (4K)</option>
                  </select>
                </div>
                <div>
                  <label className="block text-[12px] text-white/50 mb-2">Bitrate</label>
                  <input
                    type="number"
                    className={inputCls}
                    value={settings.video.bitrate}
                    onChange={(e) =>
                      update((s) => ({
                        ...s,
                        video: { ...s.video, bitrate: Number(e.target.value) || s.video.bitrate },
                      }))
                    }
                  />
                  <p className="text-[10px] text-white/15 mt-1">Em bps (2000000 = 2 Mbps)</p>
                </div>
                <div>
                  <label className="block text-[12px] text-white/50 mb-2">Container</label>
                  <select
                    className={selectCls}
                    value={settings.video.container}
                    onChange={(e) =>
                      update((s) => ({
                        ...s,
                        video: { ...s.video, container: e.target.value },
                      }))
                    }
                  >
                    <option value="mkv">MKV</option>
                    <option value="mp4">MP4</option>
                  </select>
                </div>
              </div>
              <div className="pt-4 border-t border-white/5 flex items-center gap-3">
                <button
                  className="px-6 py-2.5 bg-brand-500 hover:bg-brand-600 text-white rounded-xl text-[12px] font-medium transition-all active:scale-[0.98] glow-sm border-0 cursor-pointer disabled:opacity-40"
                  onClick={handleSave}
                  disabled={saving}
                >
                  {saving ? "Salvando..." : "Salvar"}
                </button>
                {saved && <span className="text-emerald-400 text-[11px]">Salvo!</span>}
                {error && <span className="text-red-400/80 text-xs">{error}</span>}
              </div>
            </div>
          )}

          {/* Audio */}
          {activeTab === "audio" && (
            <div className="space-y-5">
              <h2 className="text-[10px] font-bold text-white/25 uppercase tracking-[0.2em] mb-4">Áudio</h2>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-[12px] text-white/50 mb-2">Codec</label>
                  <select
                    className={selectCls}
                    value={settings.audio.codec}
                    onChange={(e) =>
                      update((s) => ({
                        ...s,
                        audio: { ...s.audio, codec: e.target.value },
                      }))
                    }
                  >
                    <option value="opus">Opus</option>
                    <option value="aac">AAC</option>
                    <option value="flac">FLAC</option>
                  </select>
                </div>
                <div>
                  <label className="block text-[12px] text-white/50 mb-2">Bitrate</label>
                  <input
                    type="number"
                    className={inputCls}
                    value={settings.audio.bitrate}
                    onChange={(e) =>
                      update((s) => ({
                        ...s,
                        audio: { ...s.audio, bitrate: Number(e.target.value) || s.audio.bitrate },
                      }))
                    }
                  />
                </div>
              </div>
              <div className="pt-4 border-t border-white/5 flex items-center gap-3">
                <button
                  className="px-6 py-2.5 bg-brand-500 hover:bg-brand-600 text-white rounded-xl text-[12px] font-medium transition-all active:scale-[0.98] glow-sm border-0 cursor-pointer disabled:opacity-40"
                  onClick={handleSave}
                  disabled={saving}
                >
                  {saving ? "Salvando..." : "Salvar"}
                </button>
                {saved && <span className="text-emerald-400 text-[11px]">Salvo!</span>}
                {error && <span className="text-red-400/80 text-xs">{error}</span>}
              </div>
            </div>
          )}

          {/* Transcrição */}
          {activeTab === "trans" && (
            <div className="space-y-5">
              <h2 className="text-[10px] font-bold text-white/25 uppercase tracking-[0.2em] mb-4">Transcrição</h2>
              <div className="flex items-center justify-between">
                <span className="text-[12px] text-white/50">Provedor</span>
                <div className="flex bg-white/[0.03] rounded-xl p-1 border border-white/5">
                  <button
                    onClick={() =>
                      update((s) => ({
                        ...s,
                        transcription: { ...s.transcription, provider: "local" },
                      }))
                    }
                    className={`px-4 py-1.5 text-[10px] rounded-lg border-0 cursor-pointer transition-all ${
                      settings.transcription.provider === "local"
                        ? "bg-brand-500 text-white font-medium"
                        : "text-white/30 hover:text-white/50 bg-transparent"
                    }`}
                  >
                    Local
                  </button>
                  <button
                    onClick={() =>
                      update((s) => ({
                        ...s,
                        transcription: { ...s.transcription, provider: "api" },
                      }))
                    }
                    className={`px-4 py-1.5 text-[10px] rounded-lg border-0 cursor-pointer transition-all ${
                      settings.transcription.provider === "api"
                        ? "bg-brand-500 text-white font-medium"
                        : "text-white/30 hover:text-white/50 bg-transparent"
                    }`}
                  >
                    API
                  </button>
                </div>
              </div>

              {settings.transcription.provider === "local" && (
                <div>
                  <label className="block text-[12px] text-white/50 mb-2">Modelo</label>
                  {models.length > 0 ? (
                    <div className="flex flex-col gap-2">
                      {models.map((m) => (
                        <div key={m.name} className="flex justify-between items-center glass-input rounded-xl px-4 py-2.5">
                          <div className="flex gap-4 items-center">
                            <span className="text-[12px] font-medium text-white/70">{m.name}</span>
                            <span className="text-[10px] text-white/30">
                              {(m.size_bytes / 1_000_000).toFixed(0)} MB
                            </span>
                          </div>
                          <div className="shrink-0">
                            {m.downloaded ? (
                              <button
                                className={`px-3 py-1 text-[10px] rounded-lg cursor-pointer border-0 transition-all ${
                                  activeModelName === m.name
                                    ? "bg-emerald-500/15 text-emerald-400 cursor-default"
                                    : "bg-transparent text-white/40 hover:text-white/70"
                                }`}
                                onClick={() => handleSetActiveModel(m.name)}
                                disabled={activeModelName === m.name}
                              >
                                {activeModelName === m.name ? "Ativo" : "Usar"}
                              </button>
                            ) : (
                              <button
                                className="px-3 py-1 text-[10px] rounded-lg cursor-pointer bg-brand-500 text-white border-0 hover:bg-brand-600 disabled:opacity-40 disabled:cursor-not-allowed"
                                onClick={() => handleDownloadModel(m.name)}
                                disabled={downloadingModel !== null}
                              >
                                {downloadingModel === m.name ? "Baixando..." : "Baixar"}
                              </button>
                            )}
                          </div>
                        </div>
                      ))}
                    </div>
                  ) : (
                    <select
                      className={selectCls}
                      value={settings.transcription.model}
                      onChange={(e) =>
                        update((s) => ({
                          ...s,
                          transcription: { ...s.transcription, model: e.target.value },
                        }))
                      }
                    >
                      <option value="tiny">tiny (75 MB)</option>
                      <option value="base">base (150 MB)</option>
                      <option value="small">small (500 MB)</option>
                      <option value="medium">medium (1.5 GB)</option>
                      <option value="large">large (3 GB)</option>
                    </select>
                  )}
                </div>
              )}

              {settings.transcription.provider === "api" && (
                <>
                  <div>
                    <label className="block text-[12px] text-white/50 mb-2">URL da API</label>
                    <input
                      type="text"
                      className={inputCls}
                      value={settings.transcription.api_url}
                      onChange={(e) =>
                        update((s) => ({
                          ...s,
                          transcription: { ...s.transcription, api_url: e.target.value },
                        }))
                      }
                      placeholder="https://api.openai.com/v1"
                    />
                  </div>
                  <div>
                    <label className="block text-[12px] text-white/50 mb-2">Chave da API</label>
                    <input
                      type="password"
                      className={inputCls}
                      value={settings.transcription.api_key}
                      onChange={(e) =>
                        update((s) => ({
                          ...s,
                          transcription: { ...s.transcription, api_key: e.target.value },
                        }))
                      }
                      placeholder="sk-..."
                    />
                  </div>
                  <div>
                    <label className="block text-[12px] text-white/50 mb-2">Modelo</label>
                    <input
                      type="text"
                      className={inputCls}
                      value={settings.transcription.model}
                      onChange={(e) =>
                        update((s) => ({
                          ...s,
                          transcription: { ...s.transcription, model: e.target.value },
                        }))
                      }
                      placeholder="whisper-1"
                    />
                  </div>
                </>
              )}

              <div className="pt-4 border-t border-white/5 flex items-center gap-3">
                <button
                  className="px-6 py-2.5 bg-brand-500 hover:bg-brand-600 text-white rounded-xl text-[12px] font-medium transition-all active:scale-[0.98] glow-sm border-0 cursor-pointer disabled:opacity-40"
                  onClick={handleSave}
                  disabled={saving}
                >
                  {saving ? "Salvando..." : "Salvar"}
                </button>
                {saved && <span className="text-emerald-400 text-[11px]">Salvo!</span>}
                {error && <span className="text-red-400/80 text-xs">{error}</span>}
              </div>
            </div>
          )}

          {/* RAG */}
          {activeTab === "rag" && (
            <div className="space-y-5">
              <h2 className="text-[10px] font-bold text-white/25 uppercase tracking-[0.2em] mb-4">RAG / Chat</h2>
              <div>
                <label className="block text-[12px] text-white/50 mb-2">URL de Embeddings</label>
                <input
                  type="text"
                  className={inputCls}
                  value={settings.rag.embeddings_url}
                  onChange={(e) =>
                    update((s) => ({
                      ...s,
                      rag: { ...s.rag, embeddings_url: e.target.value },
                    }))
                  }
                  placeholder="https://openrouter.ai/api/v1"
                />
                <p className="text-[10px] text-white/15 mt-1">URL base — /embeddings será adicionado automaticamente</p>
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-[12px] text-white/50 mb-2">API Key</label>
                  <input
                    type="password"
                    className={inputCls}
                    value={settings.rag.embeddings_api_key}
                    onChange={(e) =>
                      update((s) => ({
                        ...s,
                        rag: { ...s.rag, embeddings_api_key: e.target.value },
                      }))
                    }
                    placeholder="sk-..."
                  />
                </div>
                <div>
                  <label className="block text-[12px] text-white/50 mb-2">Modelo</label>
                  <input
                    type="text"
                    className={inputCls}
                    value={settings.rag.embeddings_model}
                    onChange={(e) =>
                      update((s) => ({
                        ...s,
                        rag: { ...s.rag, embeddings_model: e.target.value },
                      }))
                    }
                    placeholder="text-embedding-3-small"
                  />
                </div>
              </div>

              <div className="pt-3 border-t border-white/5">
                <div>
                  <label className="block text-[12px] text-white/50 mb-2">URL do Chat</label>
                  <input
                    type="text"
                    className={inputCls}
                    value={settings.rag.chat_url}
                    onChange={(e) =>
                      update((s) => ({
                        ...s,
                        rag: { ...s.rag, chat_url: e.target.value },
                      }))
                    }
                    placeholder="https://openrouter.ai/api/v1"
                  />
                  <p className="text-[10px] text-white/15 mt-1">URL base — /chat/completions será adicionado automaticamente</p>
                </div>
              </div>
              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-[12px] text-white/50 mb-2">Chave chat</label>
                  <input
                    type="password"
                    className={inputCls}
                    value={settings.rag.chat_api_key}
                    onChange={(e) =>
                      update((s) => ({
                        ...s,
                        rag: { ...s.rag, chat_api_key: e.target.value },
                      }))
                    }
                  />
                </div>
                <div>
                  <label className="block text-[12px] text-white/50 mb-2">Modelo chat</label>
                  <input
                    type="text"
                    className={inputCls}
                    value={settings.rag.chat_model}
                    onChange={(e) =>
                      update((s) => ({
                        ...s,
                        rag: { ...s.rag, chat_model: e.target.value },
                      }))
                    }
                    placeholder="openai/gpt-4o-mini"
                  />
                </div>
              </div>

              <div className="grid grid-cols-2 gap-4 pt-3 border-t border-white/5">
                <div>
                  <label className="block text-[12px] text-white/50 mb-2">Tamanho do chunk</label>
                  <input
                    type="number"
                    className={inputCls}
                    value={settings.rag.chunk_size}
                    onChange={(e) =>
                      update((s) => ({
                        ...s,
                        rag: { ...s.rag, chunk_size: Number(e.target.value) || s.rag.chunk_size },
                      }))
                    }
                  />
                </div>
                <div>
                  <label className="block text-[12px] text-white/50 mb-2">Top-K resultados</label>
                  <input
                    type="number"
                    className={inputCls}
                    value={settings.rag.top_k}
                    onChange={(e) =>
                      update((s) => ({
                        ...s,
                        rag: { ...s.rag, top_k: Number(e.target.value) || s.rag.top_k },
                      }))
                    }
                  />
                </div>
              </div>

              <div className="pt-4 border-t border-white/5 flex items-center gap-3">
                <button
                  className="px-6 py-2.5 bg-brand-500 hover:bg-brand-600 text-white rounded-xl text-[12px] font-medium transition-all active:scale-[0.98] glow-sm border-0 cursor-pointer disabled:opacity-40"
                  onClick={handleSave}
                  disabled={saving}
                >
                  {saving ? "Salvando..." : "Salvar"}
                </button>
                {saved && <span className="text-emerald-400 text-[11px]">Salvo!</span>}
                {error && <span className="text-red-400/80 text-xs">{error}</span>}
              </div>
            </div>
          )}
        </div>
      </div>
    </>
  );
}
