window.addEventListener("unhandledrejection", function (event) {
    console.warn("🚨 Unhandled promise rejection:", event.reason);
});
