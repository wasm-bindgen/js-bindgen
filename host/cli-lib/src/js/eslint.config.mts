import js from "@eslint/js"
import { defineConfig, type Config } from "eslint/config"
import tseslint from "typescript-eslint"
import shared from "../../../eslint.config.base.mts"

export default defineConfig(
	{
		ignores: ["eslint.config.mts", "**/*.mjs", "**/*.d.mts"],
	},
	js.configs.recommended,
	tseslint.configs.strictTypeChecked,
	tseslint.configs.stylisticTypeChecked,
	shared as Config,
	{
		languageOptions: {
			parserOptions: {
				projectService: true,
			},
		},
	}
)
