const iframes = Array.prototype.slice.apply(document.getElementsByTagName("iframe"));
for (const [i, iframe] of iframes.entries()) {
    iframe.style.height = iframe.getBoundingClientRect().height + "px";

    iframe.addEventListener('load', () => {
        const innerBody = window.frames[i].document.body;
        innerBody.style.overflow = "hidden";

        const resize = () => {
            if (innerBody.scrollHeight == 0) {
                window.setTimeout(resize, 50);
                return;
            }
            iframe.style.height = innerBody.scrollHeight + "px";
        }

        window.setTimeout(resize, 50);
    });
}
