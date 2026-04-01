<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { fade, fly, slide, scale } from 'svelte/transition';
  import { quintOut, elasticOut } from 'svelte/easing';

  let config = { xampp_path: '', backup_path: '' };
  let status = 'Ready';
  let progress = 0;
  let logs: { id: number; msg: string; type: 'info' | 'error' | 'success' }[] = [];
  let logId = 0;
  let isDarkMode = true;
  let isSplashVisible = true;

  interface Payload {
    message: string;
    progress: number;
    status: string;
  }

  function addLog(msg: string, type: 'info' | 'error' | 'success' = 'info') {
    logs = [{ id: logId++, msg, type }, ...logs].slice(0, 50);
  }

  onMount(() => {
    // Hide splash screen after 2.5 seconds
    setTimeout(() => {
      isSplashVisible = false;
    }, 2500);

    // Dark mode detection
    isDarkMode = window.matchMedia('(prefers-color-scheme: dark)').matches;
    
    const initAsync = async () => {
      try {
        config = await invoke('get_config');
        addLog('System initialized.', 'success');
      } catch (e) {
        addLog(`Init Error: ${e}`, 'error');
      }

      const unlistenLog = await listen<Payload>('log', (event) => {
        const type = event.payload.status === 'Error' ? 'error' : (event.payload.progress >= 1 ? 'success' : 'info');
        addLog(event.payload.message, type);
        status = event.payload.status;
        progress = event.payload.progress;
      });

      const unlistenProgress = await listen<Payload>('progress', (event) => {
        progress = event.payload.progress;
        status = event.payload.status;
      });

      return () => {
        unlistenLog();
        unlistenProgress();
      };
    };

    initAsync();
  });

  async function startBackup() {
    try {
      await invoke('start_backup');
    } catch (e) {
      addLog(`Backup Error: ${e}`, 'error');
    }
  }

  async function startRestore() {
    try {
      await invoke('start_restore');
    } catch (e) {
      addLog(`Restore Error: ${e}`, 'error');
    }
  }

  async function startInstall() {
    try {
      await invoke('start_install');
    } catch (e) {
      addLog(`Install Error: ${e}`, 'error');
    }
  }

  async function checkUpdates() {
    try {
      await invoke('check_updates');
    } catch (e) {
      addLog(`Update Error: ${e}`, 'error');
    }
  }

  async function discoverXampp() {
    try {
      const path = await invoke<string | null>('discover_xampp');
      if (path) {
        config = { ...config, xampp_path: path };
        addLog(`XAMPP discovered at: ${path}`, 'success');
      } else {
        addLog(`Discovery failed.`, 'error');
      }
    } catch (e) {
      addLog(`Discovery Error: ${e}`, 'error');
    }
  }

  function toggleTheme() {
    isDarkMode = !isDarkMode;
  }
</script>

{#if isSplashVisible}
  <div class="splash-screen" out:scale={{ duration: 1000, easing: quintOut, start: 1, opacity: 0 }}>
    <div class="splash-content">
      <div class="splash-logo" in:scale={{ duration: 1500, delay: 200, easing: elasticOut }}>
        <span class="xampp">XAMPP</span>&nbsp;<span class="util">UTILITY</span>
      </div>
      <div class="splash-loader" in:fade={{ delay: 800, duration: 1000 }}>
        <div class="bar"></div>
      </div>
      <p class="splash-tagline" in:fly={{ y: 20, delay: 1000, duration: 800 }}>Initializing your environment...</p>
    </div>
  </div>
{/if}

<div class="app-shell" class:dark={isDarkMode}>
  <div class="background-blob blob-1"></div>
  <div class="background-blob blob-2"></div>
  
  <main class="container">
    {#if !isSplashVisible}
      <header in:fly={{ y: -50, duration: 800, easing: quintOut }}>
        <div class="title-group">
          <h1>XAMPP <span>Utility</span></h1>
          <p class="subtitle">Next-Gen Environment Management</p>
        </div>
        <button class="theme-toggle" on:click={toggleTheme} aria-label="Toggle Theme">
          {#if isDarkMode}
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/><line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/></svg>
          {:else}
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg>
          {/if}
        </button>
      </header>

      <div class="grid">
        <!-- Config Section -->
        <section class="card glass" in:fly={{ x: -50, delay: 200, duration: 800, easing: quintOut }}>
          <div class="card-header">
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.1a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>
            <h2>Environment</h2>
          </div>
          <div class="config-content">
            <div class="path-group">
              <label for="xampp-path">XAMPP Core</label>
              <div class="path-value" id="xampp-path">
                <span>{config.xampp_path || 'Location unknown'}</span>
                <button class="action-btn small" on:click={discoverXampp}>Auto-Detect</button>
              </div>
            </div>
            <div class="path-group">
              <label for="backup-path">Backup Vault</label>
              <div class="path-value" id="backup-path">
                <span>{config.backup_path || 'No vault set'}</span>
              </div>
            </div>
          </div>
        </section>

        <!-- Stats Section -->
        <section class="card glass status-card" in:fly={{ x: 50, delay: 400, duration: 800, easing: quintOut }}>
          <div class="card-header">
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 20v-6M6 20V10M18 20V4"/></svg>
            <h2>Live Status</h2>
          </div>
          <div class="status-display">
            <div class="status-badge" class:active={status !== 'Ready' && status !== 'Error'}>
              {status}
            </div>
            <div class="progress-wrapper">
              <div class="progress-text">{Math.round(progress * 100)}%</div>
              <div class="progress-outer">
                <div class="progress-inner" style="width: {progress * 100}%"></div>
              </div>
            </div>
          </div>
        </section>
      </div>

      <!-- Quick Actions -->
      <section class="actions-strip glass" in:fly={{ y: 50, delay: 600, duration: 800, easing: quintOut }}>
        <button class="glow-btn" on:click={startInstall}>
          <span class="icon">🚀</span>
          <span>Install</span>
        </button>
        <button class="glow-btn" on:click={startBackup}>
          <span class="icon">💾</span>
          <span>Backup</span>
        </button>
        <button class="glow-btn" on:click={startRestore}>
          <span class="icon">🔄</span>
          <span>Restore</span>
        </button>
        <button class="glow-btn" on:click={checkUpdates}>
          <span class="icon">✨</span>
          <span>Updates</span>
        </button>
      </section>

      <!-- Interactive Log -->
      <section class="log-container glass" in:fly={{ y: 50, delay: 800, duration: 800, easing: quintOut }}>
        <div class="card-header">
          <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4 22h16a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2H8a2 2 0 0 0-2 2v16a2 2 0 0 1-2 2Zm0 0a2 2 0 0 1-2-2v-9c0-1.1.9-2 2-2h2"/><path d="M18 14h-8"/><path d="M15 18h-5"/><path d="M10 6h8v4h-8V6Z"/></svg>
          <h2>Activity Feed</h2>
        </div>
        <div class="log-viewport">
          {#each logs as log (log.id)}
            <div 
              class="log-entry {log.type}" 
              transition:slide={{ duration: 300, easing: quintOut }}
            >
              <span class="timestamp">[{new Date().toLocaleTimeString()}]</span>
              <span class="message">{log.msg}</span>
            </div>
          {/each}
          {#if logs.length === 0}
            <div class="empty-state" transition:fade>No recent activity detected.</div>
          {/if}
        </div>
      </section>
    {/if}
  </main>
</div>

<style>
  :global(html, body) {
    margin: 0;
    padding: 0;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    user-select: none;
    background-color: #0f172a; /* Fallback for splash */
  }

  /* Splash Screen */
  .splash-screen {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: linear-gradient(135deg, #0f172a 0%, #1e1b4b 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
    color: white;
  }

  .splash-content {
    text-align: center;
  }

  .splash-logo {
    font-size: 4rem;
    font-weight: 900;
    letter-spacing: -2px;
    margin-bottom: 2rem;
  }

  .splash-logo .xampp { color: #ffffff; }
  .splash-logo .util { color: #6366f1; text-shadow: 0 0 30px rgba(99, 102, 241, 0.5); }

  .splash-loader {
    width: 200px;
    height: 4px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 10px;
    margin: 0 auto 1.5rem;
    overflow: hidden;
  }

  .splash-loader .bar {
    width: 100%;
    height: 100%;
    background: #6366f1;
    transform: translateX(-100%);
    animation: loading 2.5s cubic-bezier(0.4, 0, 0.2, 1) forwards;
  }

  @keyframes loading {
    0% { transform: translateX(-100%); }
    50% { transform: translateX(-30%); }
    100% { transform: translateX(0%); }
  }

  .splash-tagline {
    font-size: 0.9rem;
    opacity: 0.6;
    text-transform: uppercase;
    letter-spacing: 3px;
  }

  :root {
    --primary: #6366f1;
    --primary-glow: rgba(99, 102, 241, 0.4);
    --bg-light: #f8fafc;
    --bg-dark: #0f172a;
    --text-light: #1e293b;
    --text-dark: #f1f5f9;
    --glass-light: rgba(255, 255, 255, 0.7);
    --glass-dark: rgba(30, 41, 59, 0.7);
    --border-light: rgba(255, 255, 255, 0.3);
    --border-dark: rgba(255, 255, 255, 0.1);
    --success: #10b981;
    --error: #f43f5e;
    --info: #3b82f6;
  }

  .app-shell {
    height: 100vh;
    width: 100vw;
    transition: background-color 0.5s ease, color 0.5s ease;
    background-color: var(--bg-light);
    color: var(--text-light);
    overflow: hidden;
    position: relative;
  }

  .app-shell.dark {
    background-color: var(--bg-dark);
    color: var(--text-dark);
  }

  /* Animated background blobs */
  .background-blob {
    position: fixed;
    width: 60vw;
    height: 60vw;
    z-index: 0;
    pointer-events: none;
    filter: blur(80px);
    opacity: 0.4;
  }

  .blob-1 {
    top: -20%;
    right: -20%;
    background: radial-gradient(circle, var(--primary) 0%, transparent 70%);
    animation: blob1 20s infinite alternate;
  }

  .blob-2 {
    bottom: -20%;
    left: -20%;
    background: radial-gradient(circle, #3b82f6 0%, transparent 70%);
    animation: blob2 25s infinite alternate-reverse;
  }

  @keyframes blob1 {
    0% { transform: translate(0, 0) scale(1); }
    100% { transform: translate(-20%, 20%) scale(1.2); }
  }

  @keyframes blob2 {
    0% { transform: translate(0, 0) scale(1.2); }
    100% { transform: translate(20%, -20%) scale(1); }
  }

  .container {
    position: relative;
    z-index: 1;
    max-width: 1000px;
    height: 100%;
    margin: 0 auto;
    padding: 1.5rem 2rem;
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    flex-shrink: 0;
  }

  h1 {
    font-size: 2.2rem;
    font-weight: 800;
    margin: 0;
    letter-spacing: -1px;
  }

  h1 span {
    color: var(--primary);
    position: relative;
  }

  .subtitle {
    margin: 0.1rem 0 0;
    opacity: 0.6;
    font-size: 0.85rem;
    animation: subtitleTrack 10s infinite ease-in-out;
  }

  @keyframes subtitleTrack {
    0%, 100% { letter-spacing: 0.5px; }
    50% { letter-spacing: 2px; }
  }

  .theme-toggle {
    background: none;
    border: 2px solid currentColor;
    border-radius: 50%;
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
    opacity: 0.8;
  }

  .theme-toggle:hover {
    transform: rotate(180deg) scale(1.1);
    background-color: var(--primary);
    border-color: var(--primary);
    color: white;
    box-shadow: 0 0 20px var(--primary-glow);
  }

  .grid {
    display: grid;
    grid-template-columns: 1.5fr 1fr;
    gap: 1.25rem;
    margin-bottom: 1.25rem;
    flex-shrink: 0;
  }

  .glass {
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border: 1px solid var(--border-light);
    background: var(--glass-light);
    box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.1);
  }

  .dark .glass {
    border: 1px solid var(--border-dark);
    background: var(--glass-dark);
  }

  .card {
    border-radius: 20px;
    padding: 1.25rem;
    transition: all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  }

  .card:hover {
    transform: translateY(-5px) scale(1.01);
    box-shadow: 0 15px 45px rgba(0,0,0,0.1);
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 1rem;
    opacity: 0.9;
  }

  .card-header h2 {
    font-size: 1rem;
    font-weight: 700;
    margin: 0;
    text-transform: uppercase;
    letter-spacing: 1px;
  }

  .path-group {
    margin-bottom: 0.75rem;
  }

  .path-group label {
    display: block;
    font-size: 0.7rem;
    font-weight: 700;
    text-transform: uppercase;
    opacity: 0.5;
    margin-bottom: 0.3rem;
  }

  .path-value {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: rgba(0,0,0,0.05);
    padding: 0.4rem 0.6rem;
    border-radius: 8px;
    font-family: ui-monospace, monospace;
    font-size: 0.8rem;
    overflow: hidden;
    transition: all 0.3s ease;
  }

  .dark .path-value {
    background: rgba(255,255,255,0.05);
  }

  .path-value:hover {
    background: rgba(99, 102, 241, 0.1);
  }

  .action-btn.small {
    background: var(--primary);
    color: white;
    border: none;
    padding: 0.2rem 0.5rem;
    border-radius: 5px;
    font-size: 0.65rem;
    font-weight: 700;
    cursor: pointer;
    transition: all 0.2s ease;
    flex-shrink: 0;
  }

  .action-btn.small:hover {
    filter: brightness(1.2);
    transform: scale(1.05);
  }

  .status-badge {
    display: inline-block;
    padding: 0.4rem 0.8rem;
    border-radius: 50px;
    background: rgba(0,0,0,0.1);
    font-weight: 700;
    font-size: 1rem;
    margin-bottom: 1rem;
    transition: all 0.5s ease;
  }

  .status-badge.active {
    background: var(--primary);
    color: white;
    box-shadow: 0 0 20px var(--primary-glow);
    animation: breath 2s infinite ease-in-out;
  }

  @keyframes breath {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.8; transform: scale(1.05); }
  }

  .progress-outer {
    height: 10px;
    background: rgba(0,0,0,0.1);
    border-radius: 10px;
    overflow: hidden;
  }

  .progress-inner {
    height: 100%;
    background: linear-gradient(90deg, var(--primary), #818cf8);
    transition: width 0.8s cubic-bezier(0.34, 1.56, 0.64, 1);
    box-shadow: 0 0 10px var(--primary-glow);
  }

  .actions-strip {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 1rem;
    padding: 0.75rem;
    border-radius: 20px;
    margin-bottom: 1.25rem;
    flex-shrink: 0;
  }

  .glow-btn {
    background: transparent;
    border: 1px solid var(--border-light);
    color: inherit;
    padding: 0.75rem;
    border-radius: 15px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.35rem;
    cursor: pointer;
    transition: all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
    font-weight: 700;
    font-size: 0.9rem;
  }

  .glow-btn:hover {
    background: var(--primary);
    color: white;
    transform: translateY(-8px) scale(1.05);
    box-shadow: 0 15px 30px var(--primary-glow);
    border-color: transparent;
  }

  .log-container {
    border-radius: 20px;
    padding: 1.25rem;
    flex-grow: 1;
    min-height: 0;
    display: flex;
    flex-direction: column;
  }

  .log-viewport {
    flex-grow: 1;
    overflow-y: auto;
    padding-right: 0.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  .log-entry {
    padding: 0.6rem 0.8rem;
    border-radius: 10px;
    background: rgba(0,0,0,0.03);
    font-size: 0.85rem;
    display: flex;
    gap: 0.6rem;
    border-left: 3px solid var(--info);
    animation: entrySlide 0.4s ease-out;
  }

  @keyframes entrySlide {
    from { transform: translateX(-10px); opacity: 0; }
    to { transform: translateX(0); opacity: 1; }
  }

  .log-entry.success { border-left-color: var(--success); }
  .log-entry.error { border-left-color: var(--error); background: rgba(244, 63, 94, 0.1); }
</style>
