/* ============================================================
   Genesis Research Engine — Interactive Documentary
   Script: Scroll Reveal + Counter Animation + TTS Narration
   ============================================================ */

(function () {
  'use strict';

  // ---- Intersection Observer: Reveal on Scroll ----
  const revealElements = document.querySelectorAll('.reveal');

  const revealObserver = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          entry.target.classList.add('visible');
          revealObserver.unobserve(entry.target);
        }
      });
    },
    { threshold: 0.12, rootMargin: '0px 0px -40px 0px' }
  );

  revealElements.forEach((el) => revealObserver.observe(el));

  // ---- Counter Animation (hero stats) ----
  const counterElements = document.querySelectorAll('[data-count]');
  let countersAnimated = false;

  function animateCounters() {
    if (countersAnimated) return;
    countersAnimated = true;

    counterElements.forEach((el) => {
      const target = parseInt(el.getAttribute('data-count'), 10);
      const duration = 2000;
      const start = performance.now();

      function tick(now) {
        const elapsed = now - start;
        const progress = Math.min(elapsed / duration, 1);
        const eased = 1 - Math.pow(1 - progress, 3);
        const current = Math.round(eased * target);
        el.textContent = current.toLocaleString();
        if (progress < 1) requestAnimationFrame(tick);
      }

      requestAnimationFrame(tick);
    });
  }

  const heroSection = document.getElementById('hero');
  if (heroSection) {
    const heroObserver = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting) {
          setTimeout(animateCounters, 1400);
          heroObserver.disconnect();
        }
      },
      { threshold: 0.3 }
    );
    heroObserver.observe(heroSection);
  }

  // ---- Active Navigation Highlighting ----
  const sections = document.querySelectorAll('section[data-section]');
  const navLinks = document.querySelectorAll('.site-nav__links a');

  const navObserver = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          const id = entry.target.getAttribute('id');
          navLinks.forEach((link) => {
            link.classList.toggle(
              'active',
              link.getAttribute('href') === '#' + id
            );
          });
        }
      });
    },
    { threshold: 0.2, rootMargin: '-56px 0px -50% 0px' }
  );

  sections.forEach((section) => navObserver.observe(section));

  // ---- Hide Nav on Scroll Down, Show on Scroll Up ----
  let lastScrollY = 0;
  const nav = document.getElementById('nav');

  function onScroll() {
    const currentY = window.scrollY;
    if (currentY > lastScrollY && currentY > 200) {
      nav.style.transform = 'translateY(-100%)';
    } else {
      nav.style.transform = 'translateY(0)';
    }
    lastScrollY = currentY;
  }

  window.addEventListener('scroll', onScroll, { passive: true });

  // ============================================================
  //  TEXT-TO-SPEECH NARRATION ENGINE (Web Speech API)
  // ============================================================

  console.log('[Genesis] TTS engine initializing...');
  console.log('[Genesis] speechSynthesis available:', !!window.speechSynthesis);

  const synth = window.speechSynthesis;
  const narrationBlocks = Array.from(document.querySelectorAll('.narration[data-audio]'));
  const audioToggle = document.getElementById('audioToggle');
  const player = document.getElementById('narrationPlayer');
  const npPlay = document.getElementById('npPlay');
  const npStop = document.getElementById('npStop');
  const npPrev = document.getElementById('npPrev');
  const npNext = document.getElementById('npNext');
  const npSection = document.getElementById('npSection');
  const npProgressBar = document.getElementById('npProgressBar');
  const npVoice = document.getElementById('npVoice');
  const npSpeedVal = document.getElementById('npSpeedVal');
  const npSlower = document.getElementById('npSlower');
  const npFaster = document.getElementById('npFaster');
  const iconPlay = document.querySelector('.np-icon-play');
  const iconPause = document.querySelector('.np-icon-pause');

  let voices = [];
  let selectedVoice = null;
  let speechRate = 1.0;
  let currentBlockIndex = -1;
  let isPlaying = false;
  let isPaused = false;
  let currentUtterance = null;

  // ---- Extract speakable text from a narration block ----
  function getBlockText(block) {
    // Get only <p> tags (skip labels and voice notes)
    const paragraphs = block.querySelectorAll('p');
    return Array.from(paragraphs).map(p => p.textContent.trim()).join(' ... ');
  }

  // ---- Section name from data-audio attribute ----
  function getBlockName(block) {
    const label = block.querySelector('.narration__label');
    return label ? label.textContent.replace(/🎙\s*/, '').trim() : block.dataset.audio;
  }

  // ---- Populate voice selector (neural/cloud voices ONLY) ----
  function loadVoices() {
    if (!synth || !npVoice) return;
    voices = synth.getVoices();
    if (!voices.length) return;

    npVoice.innerHTML = '';

    // Only allow high-quality neural/cloud voices.
    // Robotic "Desktop" SAPI5 voices are excluded entirely.
    const qualityPatterns = [
      /Online/i,                // Microsoft neural voices (e.g. "Microsoft Andrew Online")
      /^Google/i,               // Google cloud voices
      /Natural/i,               // "Natural" tagged voices (newer browsers)
      /^Samantha$/i,            // macOS premium
      /^Alex$/i,                // macOS premium
      /^Daniel$/i,              // macOS UK
      /^Karen$/i,               // macOS AU
      /^Moira$/i,               // macOS IE
      /^Tessa$/i                // macOS ZA
    ];

    // Explicit block list — never show these
    const blockPatterns = [
      /Desktop/i,               // Old SAPI5 robotic voices
      /Mobile/i,                // Low-quality mobile variants
      /Compact/i                // Compact/embedded voices
    ];

    function isQualityVoice(v) {
      if (!v.lang.startsWith('en')) return false;
      const blocked = blockPatterns.some(p => p.test(v.name));
      if (blocked) return false;
      const quality = qualityPatterns.some(p => p.test(v.name));
      // Also accept: non-local (cloud) English voices not on block list
      return quality || !v.localService;
    }

    // Priority order for auto-selection
    const preferredOrder = [
      'Microsoft Andrew Online',
      'Microsoft Jenny Online',
      'Microsoft Guy Online',
      'Microsoft Aria Online',
      'Microsoft Christopher Online',
      'Google US English',
      'Google UK English Male',
      'Samantha',
      'Alex',
      'Daniel'
    ];

    const qualityVoices = voices.filter(isQualityVoice);
    const sorted = [];

    // Add preferred voices first (in exact priority order)
    preferredOrder.forEach(name => {
      const found = qualityVoices.find(v => v.name.includes(name));
      if (found && !sorted.includes(found)) sorted.push(found);
    });

    // Add remaining quality voices
    qualityVoices.forEach(v => { if (!sorted.includes(v)) sorted.push(v); });

    // If zero quality voices found, fall back to ANY English voice
    // (better than nothing, but warn user)
    if (sorted.length === 0) {
      const anyEnglish = voices.filter(v => v.lang.startsWith('en'));
      anyEnglish.forEach(v => sorted.push(v));
      if (sorted.length > 0) {
        console.warn('[Genesis] No neural voices found. Falling back to standard voices. For best quality, use Microsoft Edge.');
      }
    }

    sorted.forEach((v, i) => {
      const opt = document.createElement('option');
      opt.value = i;
      // Clean display names
      let displayName = v.name
        .replace('Microsoft ', '')
        .replace(' (Natural)', '')
        .replace(' - English (United States)', '')
        .replace(' - English (United Kingdom)', ' UK');
      if (!v.localService) displayName += ' \u2601'; // cloud icon
      opt.textContent = displayName;
      opt.dataset.voiceName = v.name;
      npVoice.appendChild(opt);
    });

    // Store the sorted quality list for selection
    npVoice._voices = sorted;

    // Auto-select first (highest priority) voice
    if (sorted.length > 0) {
      selectedVoice = sorted[0];
      console.log('[Genesis] Auto-selected voice:', selectedVoice.name, '| Total quality voices:', sorted.length);
    }
  }

  // Voices load asynchronously in Chrome
  if (synth) {
    loadVoices();
    if (synth.onvoiceschanged !== undefined) {
      synth.onvoiceschanged = loadVoices;
    }
    // Log loaded voices for debugging
    setTimeout(() => {
      console.log('[Genesis] Voices loaded:', voices.length, '| Selected:', selectedVoice ? selectedVoice.name : 'NONE');
      console.log('[Genesis] Player element:', !!player, '| Narration blocks:', narrationBlocks.length);
    }, 1000);
  } else {
    console.warn('[Genesis] speechSynthesis not available in this browser/context');
  }

  if (npVoice) {
    npVoice.addEventListener('change', () => {
      const idx = parseInt(npVoice.value, 10);
      const voiceList = npVoice._voices || [];
      if (voiceList[idx]) {
        selectedVoice = voiceList[idx];
      }
    });
  }

  // ---- Show/hide player bar ----
  function showPlayer() {
    if (player) {
      player.classList.add('visible');
      document.body.style.paddingBottom = '72px';
    }
  }

  function hidePlayer() {
    if (player) {
      player.classList.remove('visible');
      document.body.style.paddingBottom = '';
    }
  }

  // ---- Show narration block visually (subtitle) ----
  function showBlock(index) {
    narrationBlocks.forEach((b, i) => {
      if (i === index) {
        b.style.display = 'block';
        b.style.opacity = '0';
        b.style.transform = 'translateY(12px)';
        b.style.transition = 'none';
        void b.offsetHeight;
        b.style.transition = 'opacity 0.4s ease, transform 0.4s ease';
        b.style.opacity = '1';
        b.style.transform = 'translateY(0)';
        b.classList.add('narration--active');
      } else {
        b.style.display = 'none';
        b.classList.remove('narration--active');
      }
    });
  }

  function hideAllBlocks() {
    narrationBlocks.forEach(b => {
      b.style.display = 'none';
      b.classList.remove('narration--active');
    });
  }

  // ---- Scroll to the section containing the current narration block ----
  function scrollToBlock(block) {
    const section = block.closest('section');
    if (section) {
      const top = section.getBoundingClientRect().top + window.scrollY - 80;
      window.scrollTo({ top, behavior: 'smooth' });
    }
  }

  // ---- Update UI state ----
  function updatePlayButton() {
    if (isPlaying && !isPaused) {
      iconPlay.style.display = 'none';
      iconPause.style.display = 'block';
      npPlay.setAttribute('aria-label', 'Pause narration');
      npPlay.setAttribute('title', 'Pause');
    } else {
      iconPlay.style.display = 'block';
      iconPause.style.display = 'none';
      npPlay.setAttribute('aria-label', 'Play narration');
      npPlay.setAttribute('title', 'Play');
    }
  }

  function updateProgress() {
    const pct = narrationBlocks.length > 0
      ? ((currentBlockIndex + 1) / narrationBlocks.length) * 100
      : 0;
    npProgressBar.style.width = pct + '%';
  }

  // ---- Speak a specific block ----
  function speakBlock(index) {
    if (!synth || index < 0 || index >= narrationBlocks.length) return;

    // Cancel anything playing
    synth.cancel();

    currentBlockIndex = index;
    isPlaying = true;
    isPaused = false;

    const block = narrationBlocks[index];
    const text = getBlockText(block);
    const name = getBlockName(block);

    // Update UI
    npSection.textContent = name;
    updateProgress();
    showBlock(index);
    scrollToBlock(block);

    // Create utterance
    currentUtterance = new SpeechSynthesisUtterance(text);
    if (selectedVoice) currentUtterance.voice = selectedVoice;
    currentUtterance.rate = speechRate;
    currentUtterance.pitch = 0.95; // Slightly deeper for documentary tone
    currentUtterance.volume = 1.0;

    currentUtterance.onend = () => {
      // Auto-advance to next block
      if (currentBlockIndex < narrationBlocks.length - 1) {
        // Brief pause between sections for documentary pacing
        setTimeout(() => {
          if (isPlaying && !isPaused) {
            speakBlock(currentBlockIndex + 1);
          }
        }, 800);
      } else {
        // Finished all sections
        stopNarration();
        npSection.textContent = 'Complete';
      }
    };

    currentUtterance.onerror = (e) => {
      if (e.error !== 'canceled') {
        console.warn('Speech error:', e.error);
        npSection.textContent = 'Error: ' + e.error;
      }
    };

    updatePlayButton();
    console.log('[Genesis] Speaking block', index, ':', name, '| text length:', text.length);
    synth.speak(currentUtterance);

    // Chrome has a bug where long utterances stop after ~15 seconds.
    // Workaround: periodically call resume() to keep it alive.
    startChromeFix();
  }

  // ---- Chrome bug workaround ----
  let chromeFixInterval = null;
  function startChromeFix() {
    clearInterval(chromeFixInterval);
    chromeFixInterval = setInterval(() => {
      if (synth.speaking && !synth.paused) {
        synth.pause();
        synth.resume();
      }
    }, 10000);
  }
  function stopChromeFix() {
    clearInterval(chromeFixInterval);
    chromeFixInterval = null;
  }

  // ---- Transport controls ----
  function playNarration() {
    if (!synth) {
      npSection.textContent = 'Speech not supported';
      return;
    }

    if (isPaused) {
      // Resume from pause
      synth.resume();
      isPaused = false;
      isPlaying = true;
      updatePlayButton();
      startChromeFix();
      return;
    }

    if (isPlaying) {
      // Pause
      synth.pause();
      isPaused = true;
      updatePlayButton();
      stopChromeFix();
      return;
    }

    // Start from beginning or current position
    showPlayer();
    const startIndex = currentBlockIndex >= 0 ? currentBlockIndex : 0;
    speakBlock(startIndex);
  }

  function stopNarration() {
    synth.cancel();
    isPlaying = false;
    isPaused = false;
    currentBlockIndex = -1;
    currentUtterance = null;
    npSection.textContent = 'Ready';
    npProgressBar.style.width = '0%';
    hideAllBlocks();
    updatePlayButton();
    stopChromeFix();
  }

  function prevSection() {
    if (currentBlockIndex > 0) {
      speakBlock(currentBlockIndex - 1);
    }
  }

  function nextSection() {
    if (currentBlockIndex < narrationBlocks.length - 1) {
      speakBlock(currentBlockIndex + 1);
    }
  }

  // ---- Speed control ----
  function setSpeed(rate) {
    speechRate = Math.max(0.5, Math.min(2.0, rate));
    npSpeedVal.textContent = speechRate.toFixed(1) + '\u00D7';

    // If currently speaking, restart current block with new speed
    if (isPlaying && !isPaused && currentBlockIndex >= 0) {
      speakBlock(currentBlockIndex);
    }
  }

  // ---- Wire up event listeners ----
  if (npPlay) npPlay.addEventListener('click', playNarration);
  if (npStop) npStop.addEventListener('click', stopNarration);
  if (npPrev) npPrev.addEventListener('click', prevSection);
  if (npNext) npNext.addEventListener('click', nextSection);
  if (npSlower) npSlower.addEventListener('click', () => setSpeed(speechRate - 0.1));
  if (npFaster) npFaster.addEventListener('click', () => setSpeed(speechRate + 0.1));

  // Nav bar narration button opens the player and starts playing
  if (audioToggle) {
    audioToggle.addEventListener('click', () => {
      console.log('[Genesis] Narration button clicked. Player visible:', player && player.classList.contains('visible'), '| isPlaying:', isPlaying);

      if (!player) {
        console.error('[Genesis] Player element not found! The narration player HTML may not have loaded.');
        return;
      }

      if (!synth) {
        alert('Text-to-Speech is not supported in this browser. Please open the site in Chrome or Edge.');
        return;
      }

      if (!player.classList.contains('visible')) {
        showPlayer();
        if (!isPlaying) {
          // Small delay so player animation finishes
          setTimeout(() => speakBlock(0), 300);
        }
      } else if (isPlaying) {
        playNarration(); // toggles pause/resume
      } else {
        hidePlayer();
        hideAllBlocks();
      }
    });
  }

  // ---- Copy to clipboard for clone box ----
  document.querySelectorAll('.clone-box button').forEach((btn) => {
    btn.addEventListener('click', () => {
      const code = btn.parentElement.querySelector('code');
      if (code) {
        navigator.clipboard.writeText(code.textContent).then(() => {
          const original = btn.textContent;
          btn.textContent = 'Copied';
          setTimeout(() => { btn.textContent = original; }, 2000);
        });
      }
    });
  });

  // ---- Smooth Scroll for Nav Links ----
  document.querySelectorAll('a[href^="#"]').forEach((anchor) => {
    anchor.addEventListener('click', (e) => {
      const target = document.querySelector(anchor.getAttribute('href'));
      if (target) {
        e.preventDefault();
        const offset = 72;
        const top =
          target.getBoundingClientRect().top + window.scrollY - offset;
        window.scrollTo({ top, behavior: 'smooth' });
      }
    });
  });

  // ---- Fade scroll hint after first scroll ----
  const scrollHint = document.querySelector('.hero__scroll-hint');
  if (scrollHint) {
    window.addEventListener(
      'scroll',
      () => {
        if (window.scrollY > 100) {
          scrollHint.style.opacity = '0';
          scrollHint.style.transition = 'opacity 0.5s ease';
        }
      },
      { passive: true }
    );
  }
})();
