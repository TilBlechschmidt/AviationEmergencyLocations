import { browser } from '$app/env';
import { writable } from 'svelte/store'
import { defaultPreferences } from './data/constants';

const altitudeKey = 'altitude';
const aircraftKey = 'aircraft';
const passedDisclaimersKey = 'passedDisclaimers';
const preferencesKey = 'preferences';

const defaultDisclaimersPassed = [];
const defaultAircraft = 'C150';
const defaultAltitude = 2000;

export const altitude = writable((browser && parseInt(localStorage.getItem(altitudeKey))) || defaultAltitude);
altitude.subscribe((value) => { if (browser) localStorage.setItem(altitudeKey, value) });

export const aircraftID = writable((browser && localStorage.getItem(aircraftKey)) || defaultAircraft);
aircraftID.subscribe(aircraft => { if (browser) localStorage.setItem(aircraftKey, aircraft) });

export const passedDisclaimers = writable((browser && JSON.parse(localStorage.getItem(passedDisclaimersKey))) || defaultDisclaimersPassed);
passedDisclaimers.subscribe(value => { if (browser) localStorage.setItem(passedDisclaimersKey, JSON.stringify(value)) });

export const preferences = writable((browser && JSON.parse(localStorage.getItem(preferencesKey))) || defaultPreferences);
preferences.subscribe(value => { if (browser) localStorage.setItem(preferencesKey, JSON.stringify(value)) });
