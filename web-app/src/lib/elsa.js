import { browser } from '$app/env';
import Worker from '$lib/worker.js?worker';
import { feetToMeters } from './units';

class ElsaWorker {
    constructor() {
        this.idCounter = 0;
        this.worker = browser ? new Worker() : { postMessage: () => { } };
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

    reachabilityGeoJSON(aircraftID, altitudeInFeet) {
        const altitude = feetToMeters(altitudeInFeet);

        return this.submitRequest('REACHABILITY_GEOJSON', {
            aircraftID, altitude
        }).then(JSON.parse);
    }

    locationLinesGeoJSON(aircraftID) {
        return this.submitRequest('LOCATION_LINES_GEOJSON', {
            aircraftID
        }).then(JSON.parse);
    }

    closestLocationWithinReach(latitude, longitude, maximumDistance) {
        return this.submitRequest('CLOSEST_LOCATION_ID', {
            latitude, longitude
        })
            .then(JSON.parse)
            .then(r => r !== null && r.distance < maximumDistance ? r.location : null);
    }

    fetchLocation(locationID, aircraftID) {
        return this.submitRequest('LOCATION_DATA', { locationID, aircraftID });
    }

    fetchAircraftList() {
        return this.submitRequest('AIRCRAFT_LIST');
    }

    landingOptions(latitude, longitude, heading, altitudeInFeet, aircraftID) {
        const altitude = feetToMeters(altitudeInFeet);

        return this.submitRequest('LANDING_OPTIONS', {
            latitude, longitude, heading, altitude, aircraftID
        }).then(JSON.parse);
    }
}

export const elsa = new ElsaWorker();
