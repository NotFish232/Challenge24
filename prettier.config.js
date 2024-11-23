/** @type {import("prettier").Config} */
module.exports = {
    tabWidth: 4,
    useTabs: false,
    plugins: ["prettier-plugin-tailwindcss"],
    overrides: [
        {
            files: ["*.html.j2"],
            options: {
                parser: "html",
                htmlWhitespaceSensitivity: "ignore",
                bracketSameLine: true,
            },
        },
    ],
};