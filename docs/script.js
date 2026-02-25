/* ============================================================
   Genesis Research Engine — Interactive Documentary
   Script: Scroll Reveal + Counter Animation + Audio Toggle
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
        // Ease-out cubic
        const eased = 1 - Math.pow(1 - progress, 3);
        const current = Math.round(eased * target);

        el.textContent = current.toLocaleString();

        if (progress < 1) {
          requestAnimationFrame(tick);
        }
      }

      requestAnimationFrame(tick);
    });
  }

  // Trigger counters when hero is in view
  const heroSection = document.getElementById('hero');
  if (heroSection) {
    const heroObserver = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting) {
          // Small delay so the fade-in finishes before numbers tick
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

  // ---- Audio / Narration Toggle ----
  const audioToggle = document.getElementById('audioToggle');

  if (audioToggle) {
    audioToggle.addEventListener('click', () => {
      const isOn = document.body.classList.toggle('narration-on');

      // Update button text
      const label = audioToggle.querySelector('span');
      if (label) {
        label.textContent = isOn ? 'Hide Narration' : 'Narration';
      }

      // Toggle visual state
      audioToggle.classList.toggle('active', isOn);

      // Stagger narration block animations for storytelling pacing
      if (isOn) {
        const blocks = document.querySelectorAll('.narration[data-audio]');
        blocks.forEach((block, i) => {
          block.style.animationDelay = (i * 0.1) + 's';
        });
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
        const offset = 72; // nav height + padding
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
