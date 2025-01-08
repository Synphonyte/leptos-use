/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs", "../../src/docs/**/*.rs"],
        transform: {
            rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
        },
    },
    theme: {
        extend: {},
    },
    corePlugins: {
        preflight: false,
    },
    plugins: [
        require('@tailwindcss/forms'),
    ],
}