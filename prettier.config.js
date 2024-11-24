/** @type {import("prettier").Config} */
module.exports = {
    tabWidth: 4,
    useTabs: false,
    plugins: ["prettier-plugin-tailwindcss", "prettier-plugin-jinja-template"],
    overrides: [
        {
            files: ["*.html.j2"],
            options: {
                parser: "jinja-template",
                htmlWhitespaceSensitivity: "ignore",
                bracketSameLine: true,
            },
        },
    ],
};