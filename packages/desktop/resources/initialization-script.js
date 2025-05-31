// Clear console every 15 minutes.
setInterval(
  () => {
    console.clear();
    console.info(
      '%c[Edgebar]%c Console is cleared every 15 minutes to prevent memory buildup from logged data.',
      'color: #4ade80',
      'color: inherit',
    );
  },
  1000 * 60 * 15,
);

if (window.location.host === "127.0.0.1:6124") {
  if ("serviceWorker" in navigator) {
    navigator.serviceWorker
      .register("/__EDGEBAR/sw.js", { scope: "/" })
      .then((sw) => {
        console.info(
          '%c[EdgeBar]%c Service Worker registered.',
          'color: #4ade80',
          'color: inherit',
        );

        const message = {
          type: "SET_CONFIG",
          config: window.__EDGEBAR_STATE.config.caching,
        };

        sw.active?.postMessage(message);
        sw.installing?.postMessage(message);
        sw.waiting?.postMessage(message);
      })
      .catch((err) =>
        console.error(
          '%c[EdgeBar]%c Service Worker failed to register:',
          'color: #4ade80',
          'color: inherit',
          err,
        ),
      );
  }

  document.addEventListener("DOMContentLoaded", () => {
    addFavicon();
    loadCss("/__EDGEBAR/normalize.css");
  });
}

/**
 * Adds a CSS file with the given path to the head element.
 */
function loadCss(path) {
  const link = document.createElement("link");
  link.setAttribute("data-edgebar", "");
  link.rel = "stylesheet";
  link.type = "text/css";
  link.href = path;
  insertIntoHead(link);
}

/**
 * Adds a favicon to the head element if one is not already present.
 */
function addFavicon() {
  if (!document.querySelector('link[rel="icon"]')) {
    const link = document.createElement("link");
    link.setAttribute("data-edgebar", "");
    link.rel = "icon";
    link.href = "data:;";
    insertIntoHead(link);
  }
}

/**
 * Inserts the element before any other resource tags in the head element.
 * Ensures that user-defined stylesheets or favicons are prioritized over
 * EdgeBar's defaults.
 */
function insertIntoHead(element) {
  const resources = document.head.querySelectorAll("link, script, style");
  const target = resources[0]?.previousElementSibling;

  if (target) {
    target.after(element);
  } else {
    document.head.appendChild(element);
  }
}
