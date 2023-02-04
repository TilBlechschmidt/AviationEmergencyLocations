import { browser } from '$app/env';
import Worker from './worker.js?worker';
import { feetToMeters } from '../units';

const AIRCRAFT_CACHE = {};

class ElsaWorker {
    constructor() {
        this.idCounter = 0;
        this.worker = browser ? new Worker() : { postMessage: () => {} };
        this.responseHandlers = {};

        this.worker.onmessage = (msg) => {
            const { id, response } = msg.data;
            const handler = this.responseHandlers[id];

            if (typeof handler === 'function') {
                handler(response);
                delete this.responseHandlers[id];
            } else {
                console.warn('Received message with no associated response handler!', response);
            }
        };

        this.worker.onerror = (e) => {
            console.error('Worker thread threw an error:', e);
        };

        this.startup = new Promise((resolve) => {
            this.responseHandlers['startup'] = resolve;
        });
    }

    submitRequest(type, data) {
        const id = this.idCounter++;
        this.worker.postMessage({
            id,
            type,
            data
        });

        return new Promise((resolve) => {
            this.responseHandlers[id] = resolve;
        });
    }

    reachabilityGeoJSON(preferences, aircraftID, altitudeInFeet) {
        const altitude = feetToMeters(altitudeInFeet);

        return this.submitRequest('REACHABILITY_GEOJSON', {
            preferences, aircraftID, altitude
        }).then(JSON.parse);
    }

    locationLinesGeoJSON(preferences, aircraftID) {
        return this.submitRequest('LOCATION_LINES_GEOJSON', {
            preferences, aircraftID
        }).then(JSON.parse);
    }

    locationHitboxes(distance) {
        return this.submitRequest('LOCATION_HITBOXES', { distance }).then(JSON.parse);
    }

    closestLocationWithinReach(latitude, longitude, maximumDistance) {
        return this.submitRequest('CLOSEST_LOCATION_ID', {
            latitude, longitude
        })
            .then(JSON.parse)
            .then(r => r !== null && r.distance < maximumDistance ? r.location : null);
    }

    fetchLocation(preferences, locationID, aircraftID) {
        return this.submitRequest('LOCATION_DATA', { preferences, locationID, aircraftID });
    }

    fetchAircraftList() {
        return this.submitRequest('AIRCRAFT_LIST');
    }

    async fetchAircraft(aircraftID) {
        if (AIRCRAFT_CACHE.hasOwnProperty(aircraftID)) return AIRCRAFT_CACHE[aircraftID];

        const aircraft = await this.submitRequest('AIRCRAFT', { aircraftID });
        AIRCRAFT_CACHE[aircraftID] = aircraft;
        return aircraft;
    }

    landingOptions(preferences, latitude, longitude, heading, altitudeInFeet, aircraftID) {
        const altitude = feetToMeters(altitudeInFeet);

        return this.submitRequest('LANDING_OPTIONS', {
            preferences, latitude, longitude, heading, altitude, aircraftID
        }).then(JSON.parse);
    }

    takeoffProfile(aircraftID, distance) {
        return this.submitRequest('TAKEOFF_PROFILE', { aircraftID, distance }).then(JSON.parse);
    }
}

export const elsa = new ElsaWorker();
