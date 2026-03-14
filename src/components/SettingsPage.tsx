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
      setError(String(e));
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
      setError(String(e));
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
      setError(String(e));
    } finally {
      setDownloadingModel(null);
    }
  }

  async function handleSetActiveModel(name: string) {
    try {
      await setActiveModel(name);
      setActiveModelName(name);
    } catch (e) {
      setError(String(e));
    }
  }

  if (loading) {
    return (
      <div>
        <button className="btn-back" onClick={onBack}>&larr; Voltar</button>
        <div className="loading">Carregando configurações...</div>
      </div>
    );
  }

  if (!settings) {
    return (
      <div>
        <button className="btn-back" onClick={onBack}>&larr; Voltar</button>
        <p className="error-text">{error || "Falha ao carregar configurações."}</p>
      </div>
    );
  }

  const backendOptions = Object.keys(encoders);

  return (
    <div className="settings-page">
      <button className="btn-back" onClick={onBack}>&larr; Voltar</button>
      <h2>Configurações</h2>

      {/* General */}
      <section className="settings-section">
        <h3>Geral</h3>
        <div className="settings-field">
          <label>Diretório de gravações</label>
          <input
            type="text"
            value={settings.general.recordings_dir}
            onChange={(e) =>
              update((s) => ({
                ...s,
                general: { ...s.general, recordings_dir: e.target.value },
              }))
            }
          />
        </div>
        <div className="settings-field">
          <label>Nome automático de reunião</label>
          <input
            type="text"
            value={settings.general.auto_meeting_name}
            onChange={(e) =>
              update((s) => ({
                ...s,
                general: { ...s.general, auto_meeting_name: e.target.value },
              }))
            }
          />
        </div>
        <label className="toggle">
          <input
            type="checkbox"
            checked={settings.general.start_minimized}
            onChange={(e) =>
              update((s) => ({
                ...s,
                general: { ...s.general, start_minimized: e.target.checked },
              }))
            }
          />
          <span>Iniciar minimizado</span>
        </label>
      </section>

      {/* Audio */}
      <section className="settings-section">
        <h3>Áudio</h3>
        <div className="settings-field">
          <label>Codec</label>
          <select
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
        <div className="settings-field">
          <label>Bitrate (bps)</label>
          <input
            type="number"
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
      <section className="settings-section">
        <h3>Vídeo</h3>
        <div className="settings-field">
          <label>Codec</label>
          <select
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
        <div className="settings-field">
          <label>Backend</label>
          <select
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
        <div className="settings-field">
          <label>Contêiner</label>
          <select
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
        <div className="settings-field">
          <label>Bitrate (bps)</label>
          <input
            type="number"
            value={settings.video.bitrate}
            onChange={(e) =>
              update((s) => ({
                ...s,
                video: { ...s.video, bitrate: Number(e.target.value) },
              }))
            }
          />
        </div>
        <div className="settings-row">
          <div className="settings-field">
            <label>FPS</label>
            <input
              type="number"
              value={settings.video.fps}
              onChange={(e) =>
                update((s) => ({
                  ...s,
                  video: { ...s.video, fps: Number(e.target.value) },
                }))
              }
            />
          </div>
          <div className="settings-field">
            <label>Resolução</label>
            <select
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
      <section className="settings-section">
        <h3>Transcrição</h3>
        <div className="settings-field">
          <label>Provedor</label>
          <select
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
            <div className="settings-field">
              <label>URL da API</label>
              <input
                type="text"
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
            <div className="settings-field">
              <label>Chave da API</label>
              <input
                type="password"
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
            <div className="settings-field">
              <label>Modelo</label>
              <input
                type="text"
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
          <div className="settings-field">
            <label>Modelos Whisper</label>
            <div className="model-list">
              {models.map((m) => (
                <div key={m.name} className="model-item">
                  <div className="model-info">
                    <span className="model-name">{m.name}</span>
                    <span className="model-size">
                      {(m.size_bytes / 1_000_000).toFixed(0)} MB
                    </span>
                  </div>
                  <div className="model-actions">
                    {m.downloaded ? (
                      <button
                        className={`btn-small ${activeModelName === m.name ? "btn-active" : "btn-secondary"}`}
                        onClick={() => handleSetActiveModel(m.name)}
                        disabled={activeModelName === m.name}
                      >
                        {activeModelName === m.name ? "Ativo" : "Usar"}
                      </button>
                    ) : (
                      <button
                        className="btn-small btn-primary"
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
      <section className="settings-section">
        <h3>RAG / Chat</h3>
        <div className="settings-row">
          <div className="settings-field">
            <label>URL embeddings</label>
            <input
              type="text"
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
          <div className="settings-field">
            <label>Chave embeddings</label>
            <input
              type="password"
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
        <div className="settings-field">
          <label>Modelo embeddings</label>
          <input
            type="text"
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

        <div className="settings-row">
          <div className="settings-field">
            <label>URL chat</label>
            <input
              type="text"
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
          <div className="settings-field">
            <label>Chave chat</label>
            <input
              type="password"
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
        <div className="settings-field">
          <label>Modelo chat</label>
          <input
            type="text"
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

        <div className="settings-row">
          <div className="settings-field">
            <label>Tamanho do chunk</label>
            <input
              type="number"
              value={settings.rag.chunk_size}
              onChange={(e) =>
                update((s) => ({
                  ...s,
                  rag: { ...s.rag, chunk_size: Number(e.target.value) },
                }))
              }
            />
          </div>
          <div className="settings-field">
            <label>Top-K resultados</label>
            <input
              type="number"
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
      <div className="settings-save">
        <button className="btn-primary btn-large" onClick={handleSave} disabled={saving}>
          {saving ? "Salvando..." : "Salvar configurações"}
        </button>
        {saved && <span className="save-success">Configurações salvas!</span>}
        {error && <p className="error-text">{error}</p>}
      </div>
    </div>
  );
}
