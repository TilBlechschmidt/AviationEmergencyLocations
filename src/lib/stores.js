import { browser } from '$app/env';
import { writable } from 'svelte/store'
import { defaultPreferences } from './data/constants';

const altitudeKey = 'altitude';
const aircraftKey = 'aircraft';
const preferencesKey = 'preferences';
const disclaimerSeenKey = 'disclaimerSeen';

const defaultAircraft = 'C150';
const defaultAltitude = 2000;
const disclaimerSeenValue = 'absofreakinglutely';

export const altitude = writable((browser && parseInt(localStorage.getItem(altitudeKey))) || defaultAltitude);
altitude.subscribe((value) => { if (browser) localStorage.setItem(altitudeKey, value) });

export const aircraftID = writable((browser && localStorage.getItem(aircraftKey)) || defaultAircraft);
aircraftID.subscribe(aircraft => { if (browser) localStorage.setItem(aircraftKey, aircraft) });

export const disclaimerSeen = writable((browser && (localStorage.getItem(disclaimerSeenKey) === disclaimerSeenValue)) || false);
disclaimerSeen.subscribe(value => { if (browser) localStorage.setItem(disclaimerSeenKey, value === true ? disclaimerSeenValue : 'dontThinkSo') });

export const preferences = writable((browser && JSON.parse(localStorage.getItem(preferencesKey))) || defaultPreferences);
preferences.subscribe(value => { if (browser) localStorage.setItem(preferencesKey, JSON.stringify(value)) });
