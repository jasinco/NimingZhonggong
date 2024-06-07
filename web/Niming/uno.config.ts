// uno.config.ts
import { defineConfig, presetAttributify, presetUno } from "unocss";

export default defineConfig({
  rules: [
    ["vp", { height: "85%" }],
    ["mvp", { "min-height": "85%" }],
  ],
  theme: {
    colors: {
      dark: {
        bg_h: "#1d2021",
        bg: "#282828",
        bg_s: "#32302f",
        bg_1: "#3c3836",
        bg_2: "#504945",
        bg_3: "#665c54",
        bg_4: "#7c6f64",
        fg: "#fbf1c7",
        fg_1: "#ebdbb2",
        fg_2: "#d5c4a1",
        fg_3: "#bdae93",
        fg_4: "#a89984",
        gray_dim: "#a89984",
        red: "#fb4934",
        blue: "#83a598",
      },
    },
  },
});
