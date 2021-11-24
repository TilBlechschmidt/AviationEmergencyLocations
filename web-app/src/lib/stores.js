import { browser } from '$app/env';
import { writable } from 'svelte/store'

const altitudeKey = 'altitude';
const aircraftKey = 'aircraft';
const disclaimerKey = 'disclaimerRead';
const preferencesKey = 'preferences';

const defaultAircraft = 'C150';
const defaultAltitude = 2000;

export const altitude = writable((browser && parseInt(localStorage.getItem(altitudeKey))) || defaultAltitude);
altitude.subscribe((value) => { if (browser) localStorage.setItem(altitudeKey, value) });

export const aircraftID = writable((browser && localStorage.getItem(aircraftKey)) || defaultAircraft);
aircraftID.subscribe(aircraft => { if (browser) localStorage.setItem(aircraftKey, aircraft) });

export const disclaimerRead = writable((browser && sessionStorage.getItem(disclaimerKey) == "true") || false);
disclaimerRead.subscribe(value => { if (browser) sessionStorage.setItem(disclaimerKey, value) });

export const preferences = writable((browser && JSON.parse(localStorage.getItem(preferencesKey))) || null);
preferences.subscribe(value => { if (browser) localStorage.setItem(preferencesKey, JSON.stringify(value)) });
