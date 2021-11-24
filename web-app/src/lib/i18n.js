import { register, init, getLocaleFromNavigator } from 'svelte-i18n';

register('en', () => import('./locale/en.json'));
register('de', () => import('./locale/de.json'));

init({
    fallbackLocale: 'en',
    initialLocale: getLocaleFromNavigator(),
});
