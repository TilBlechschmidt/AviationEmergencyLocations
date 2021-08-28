import { browser } from '$app/env';
import { writable } from 'svelte/store'
import data from './data/generated.json';

const altitudeKey = 'altitude';
const defaultAltitude = 2000;

export const altitude = writable((browser && parseInt(localStorage.getItem(altitudeKey))) || defaultAltitude);
altitude.subscribe((value) => { if (browser) localStorage.setItem(altitudeKey, value) });

const aircraftKey = 'aircraft';
const defaultAircraft = Object.keys(data.aircrafts)[0];

export const aircraftID = writable((browser && localStorage.getItem(aircraftKey)) || defaultAircraft);
aircraftID.subscribe(aircraft => { if (browser) localStorage.setItem(aircraftKey, aircraft) });
