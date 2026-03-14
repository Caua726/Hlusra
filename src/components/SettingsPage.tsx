import { useState, useEffect } from "react";
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

export default function SettingsPage({ onBack }: Props) {
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [saved, setSaved] = useState(false);

  // Encoder probing
  const [encoders, setEncoders] = useState<Record<string, string[]>>({});

  // Whisper models
  const [models, setModels] = useState<WhisperModel[]>([]);
  const [activeModelName, setActiveModelName] = useState("");
  const [downloadingModel, setDownloadingModel] = useState<string | null>(null);

  useEffect(() => {
    loadAll();
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
    if (!settings) return;
    setSaved(false);
    setSettings(fn(settings));
  }

  async function handleSave() {
    if (!settings) return;
    setError(null);
    setSaving(true);
    setSaved(false);
    try {
      await updateSettings(settings);
      setSaved(true);
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

  const btnBack = "bg-transparent text-zinc-500 border-none py-1.5 text-sm cursor-pointer mb-4 transition-colors duration-150 hover:text-zinc-100";
  const inputCls = "w-full bg-zinc-800 text-zinc-100 border border-zinc-700 rounded-md px-2.5 py-2 text-sm font-[inherit] outline-none transition-colors duration-150 focus:border-blue-500";
  const selectCls = "w-full bg-zinc-800 text-zinc-100 border border-zinc-700 rounded-md px-2.5 py-2 text-sm font-[inherit] cursor-pointer appearance-none outline-none transition-colors duration-150 pr-8 bg-[url('data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%2212%22%20height%3D%2212%22%20viewBox%3D%220%200%2012%2012%22%3E%3Cpath%20fill%3D%22%23888%22%20d%3D%22M6%208L1%203h10z%22%2F%3E%3C%2Fsvg%3E')] bg-no-repeat bg-[right_0.65rem_center] focus:border-blue-500";

  if (loading) {
    return (
      <div>
        <button className={btnBack} onClick={onBack}>&larr; Voltar</button>
        <div className="text-center p-12 text-zinc-500">Carregando configuracoes...</div>
      </div>
    );
  }

  if (!settings) {
    return (
      <div>
        <button className={btnBack} onClick={onBack}>&larr; Voltar</button>
        <p className="text-red-500 text-sm mt-2">{error || "Falha ao carregar configuracoes."}</p>
      </div>
    );
  }

  const backendOptions = Object.keys(encoders);

  return (
    <div className="pb-12">
      <button className={btnBack} onClick={onBack}>&larr; Voltar</button>
      <h2 className="text-2xl font-semibold mb-6">Configuracoes</h2>

      {/* General */}
      <section className="bg-zinc-800/50 border border-zinc-700 rounded-xl p-5 mb-5">
        <h3 className="text-base font-semibold mb-4 text-zinc-100">Geral</h3>
        <div className="mb-3.5">
          <label className="block text-xs text-zinc-500 mb-1 font-medium">Diretorio de gravacoes</label>
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
        <div className="mb-3.5">
          <label className="block text-xs text-zinc-500 mb-1 font-medium">Nome automatico de reuniao</label>
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
        </div>
        <label className="flex items-center gap-2 text-zinc-500 text-sm cursor-pointer">
          <input
            type="checkbox"
            checked={settings.general.start_minimized}
            onChange={(e) =>
              update((s) => ({
                ...s,
                general: { ...s.general, start_minimized: e.target.checked },
              }))
            }
            className="accent-rose-500 w-4 h-4"
          />
          <span>Iniciar minimizado</span>
        </label>
      </section>

      {/* Audio */}
      <section className="bg-zinc-800/50 border border-zinc-700 rounded-xl p-5 mb-5">
        <h3 className="text-base font-semibold mb-4 text-zinc-100">Audio</h3>
        <div className="mb-3.5">
          <label className="block text-xs text-zinc-500 mb-1 font-medium">Codec</label>
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
          <label className="block text-xs text-zinc-500 mb-1 font-medium">Bitrate (bps)</label>
          <input
            type="number"
            className={inputCls}
            value={settings.audio.bitrate}
            onChange={(e) =>
              update((s) => ({
                ...s,
                audio: { ...s.audio, bitrate: Number(e.target.value) },
              }))
            }
          />
        </div>
      </section>

      {/* Video */}
      <section className="bg-zinc-800/50 border border-zinc-700 rounded-xl p-5 mb-5">
        <h3 className="text-base font-semibold mb-4 text-zinc-100">Video</h3>
        <div className="mb-3.5">
          <label className="block text-xs text-zinc-500 mb-1 font-medium">Codec</label>
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
            <option value="h264">H.264</option>
            <option value="h265">H.265</option>
            <option value="av1">AV1</option>
          </select>
        </div>
        <div className="mb-3.5">
          <label className="block text-xs text-zinc-500 mb-1 font-medium">Backend</label>
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
        <div className="mb-3.5">
          <label className="block text-xs text-zinc-500 mb-1 font-medium">Conteiner</label>
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
        <div className="mb-3.5">
          <label className="block text-xs text-zinc-500 mb-1 font-medium">Bitrate (bps)</label>
          <input
            type="number"
            className={inputCls}
            value={settings.video.bitrate}
            onChange={(e) =>
              update((s) => ({
                ...s,
                video: { ...s.video, bitrate: Number(e.target.value) },
              }))
            }
          />
        </div>
        <div className="flex gap-4 flex-wrap">
          <div className="flex-1">
            <label className="block text-xs text-zinc-500 mb-1 font-medium">FPS</label>
            <input
              type="number"
              className={inputCls}
              value={settings.video.fps}
              onChange={(e) =>
                update((s) => ({
                  ...s,
                  video: { ...s.video, fps: Number(e.target.value) },
                }))
              }
            />
          </div>
          <div className="flex-1">
            <label className="block text-xs text-zinc-500 mb-1 font-medium">Resolucao</label>
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
        </div>
      </section>

      {/* Transcription */}
      <section className="bg-zinc-800/50 border border-zinc-700 rounded-xl p-5 mb-5">
        <h3 className="text-base font-semibold mb-4 text-zinc-100">Transcricao</h3>
        <div className="mb-3.5">
          <label className="block text-xs text-zinc-500 mb-1 font-medium">Provedor</label>
          <select
            className={selectCls}
            value={settings.transcription.provider}
            onChange={(e) =>
              update((s) => ({
                ...s,
                transcription: { ...s.transcription, provider: e.target.value },
              }))
            }
          >
            <option value="local">Local (Whisper)</option>
            <option value="api">API externa</option>
          </select>
        </div>

        {settings.transcription.provider === "api" && (
          <>
            <div className="mb-3.5">
              <label className="block text-xs text-zinc-500 mb-1 font-medium">URL da API</label>
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
            <div className="mb-3.5">
              <label className="block text-xs text-zinc-500 mb-1 font-medium">Chave da API</label>
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
            <div className="mb-3.5">
              <label className="block text-xs text-zinc-500 mb-1 font-medium">Modelo</label>
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

        {settings.transcription.provider === "local" && models.length > 0 && (
          <div className="mb-3.5">
            <label className="block text-xs text-zinc-500 mb-1 font-medium">Modelos Whisper</label>
            <div className="flex flex-col gap-2">
              {models.map((m) => (
                <div key={m.name} className="flex justify-between items-center px-3 py-2 bg-zinc-800 rounded-md border border-zinc-700">
                  <div className="flex gap-4 items-center">
                    <span className="text-sm font-medium">{m.name}</span>
                    <span className="text-xs text-zinc-500">
                      {(m.size_bytes / 1_000_000).toFixed(0)} MB
                    </span>
                  </div>
                  <div className="shrink-0">
                    {m.downloaded ? (
                      <button
                        className={`px-3 py-1 text-xs rounded-md cursor-pointer border transition-colors duration-150 ${activeModelName === m.name ? "bg-green-500 text-white border-green-500 cursor-default" : "bg-transparent text-zinc-100 border-zinc-700 hover:bg-zinc-900"}`}
                        onClick={() => handleSetActiveModel(m.name)}
                        disabled={activeModelName === m.name}
                      >
                        {activeModelName === m.name ? "Ativo" : "Usar"}
                      </button>
                    ) : (
                      <button
                        className="px-3 py-1 text-xs rounded-md cursor-pointer bg-rose-500 text-white border-none hover:bg-rose-600 disabled:opacity-40 disabled:cursor-not-allowed"
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
          </div>
        )}
      </section>

      {/* RAG */}
      <section className="bg-zinc-800/50 border border-zinc-700 rounded-xl p-5 mb-5">
        <h3 className="text-base font-semibold mb-4 text-zinc-100">RAG / Chat</h3>
        <div className="flex gap-4 flex-wrap">
          <div className="flex-1 mb-3.5">
            <label className="block text-xs text-zinc-500 mb-1 font-medium">URL embeddings</label>
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
          </div>
          <div className="flex-1 mb-3.5">
            <label className="block text-xs text-zinc-500 mb-1 font-medium">Chave embeddings</label>
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
            />
          </div>
        </div>
        <div className="mb-3.5">
          <label className="block text-xs text-zinc-500 mb-1 font-medium">Modelo embeddings</label>
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
            placeholder="openai/text-embedding-3-small"
          />
        </div>

        <div className="flex gap-4 flex-wrap">
          <div className="flex-1 mb-3.5">
            <label className="block text-xs text-zinc-500 mb-1 font-medium">URL chat</label>
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
          </div>
          <div className="flex-1 mb-3.5">
            <label className="block text-xs text-zinc-500 mb-1 font-medium">Chave chat</label>
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
        </div>
        <div className="mb-3.5">
          <label className="block text-xs text-zinc-500 mb-1 font-medium">Modelo chat</label>
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

        <div className="flex gap-4 flex-wrap">
          <div className="flex-1 mb-3.5">
            <label className="block text-xs text-zinc-500 mb-1 font-medium">Tamanho do chunk</label>
            <input
              type="number"
              className={inputCls}
              value={settings.rag.chunk_size}
              onChange={(e) =>
                update((s) => ({
                  ...s,
                  rag: { ...s.rag, chunk_size: Number(e.target.value) },
                }))
              }
            />
          </div>
          <div className="flex-1 mb-3.5">
            <label className="block text-xs text-zinc-500 mb-1 font-medium">Top-K resultados</label>
            <input
              type="number"
              className={inputCls}
              value={settings.rag.top_k}
              onChange={(e) =>
                update((s) => ({
                  ...s,
                  rag: { ...s.rag, top_k: Number(e.target.value) },
                }))
              }
            />
          </div>
        </div>
      </section>

      {/* Save */}
      <div className="flex items-center gap-4 mt-2">
        <button className="bg-rose-500 text-white border-none px-12 py-4 rounded-lg text-lg cursor-pointer transition-colors duration-150 font-medium hover:bg-rose-600 disabled:opacity-40 disabled:cursor-not-allowed" onClick={handleSave} disabled={saving}>
          {saving ? "Salvando..." : "Salvar configuracoes"}
        </button>
        {saved && <span className="text-green-500 text-sm">Configuracoes salvas!</span>}
        {error && <p className="text-red-500 text-sm">{error}</p>}
      </div>
    </div>
  );
}
