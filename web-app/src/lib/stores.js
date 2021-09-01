import { browser } from '$app/env';
import { writable } from 'svelte/store'

const altitudeKey = 'altitude';
const defaultAltitude = 2000;

export const altitude = writable((browser && parseInt(localStorage.getItem(altitudeKey))) || defaultAltitude);
altitude.subscribe((value) => { if (browser) localStorage.setItem(altitudeKey, value) });

const aircraftKey = 'aircraft';
const defaultAircraft = 'C150';

export const aircraftID = writable((browser && localStorage.getItem(aircraftKey)) || defaultAircraft);
aircraftID.subscribe(aircraft => { if (browser) localStorage.setItem(aircraftKey, aircraft) });
