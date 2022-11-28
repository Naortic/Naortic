import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';
import 'dotenv/config';

console.log(`API: ${process.env.VITE_API_URL}`);

const config: UserConfig = {
	plugins: [sveltekit()]
};

export default config;
