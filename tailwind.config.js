module.exports = {
    purge: {
        mode: "all",
        content: [
            "../src/**/*.rs",
            "./index.html",
            "../src/**/*.html",
            "../src/**/*.css",
        ],
        // extract: {
        //     md: (content) => {
        //         return content.match(/[^<>"'`\s]*/)
        //     }
        // }
    },
    theme: {},
    variants: {},
    plugins: [],
};
