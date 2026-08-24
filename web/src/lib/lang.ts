import DE_DE from './lang/de-DE.json';
import EN_US from './lang/en-US.json';

type Strings = typeof EN_US;
type Key = keyof Strings;

const languages: Record<string, Record<string, string>[]> = {
	de: [DE_DE],
	'de-DE': [DE_DE],
	en: [EN_US],
	'en-US': [EN_US]
};

function getLanguage(): string {
	if (!globalThis['window']) return 'en-US';
	const lang = localStorage.getItem('nyvo.language') ?? navigator.language ?? 'en-US';
	console.log('Using language ' + lang);
	return lang;
}

export const currentLanguage = getLanguage();
const lang: Strings = {
	...EN_US,
	...languages[currentLanguage]
		.reverse()
		.flat()
		.reduce((a, c) => ({ ...a, ...c }), {})
} as Strings;

export const supportedLanguages = {
	'de-DE': 'Deutsch (Deutschland)',
	'en-US': 'English (United States)'
};

export function getFlagEmoji(lang: string): string {
	const parts = lang.split('-');
	return parts[parts.length - 1]
		.toLowerCase()
		.split('')
		.map((char) => String.fromCodePoint(0x1f185 + char.charCodeAt(0)))
		.join('');
}

export const setLanguage = (lang: keyof typeof supportedLanguages) => {
	if (!globalThis['window']) return;
	window.localStorage.setItem('nyvo.language', lang);
};

export function str(key: Key, args: Record<string, string> = {}): string {
	return lang[key].replace(/\{(\w+)\}/g, (_, k) => args[k] ?? '');
}
