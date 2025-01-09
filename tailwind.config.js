/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ["index.html", "./src/**/*.rs"],
	darkMode: "class",
	theme: {
		extend: {
			scale: {
				101: "1.01",
			},
		},
	},
	plugins: [],
};
