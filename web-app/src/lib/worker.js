import init, { Parser, Calculator, Preferences } from 'elsa';
import { dev } from '$app/env';

async function run() {
    // Start fetching everything we need
    const elsaPromise = init(dev ? undefined : '/elsa.wasm');
    const aircraftsPromise = fetch('/data/aircrafts.yml').then(res => res.text());
    const locationsPromise = fetch('/data/locations.yml').then(res => res.text());
    const dataPromise = Promise.all([aircraftsPromise, locationsPromise]);

    // Wait for all data to be fetched
    const [aircraftsYAML, locationsYAML] = await dataPromise;
    await elsaPromise;

    // Parse the fetched data
    const parser = new Parser();
    const aircrafts = parser.parseAircrafts(aircraftsYAML);
    const locations = parser.parseLocations(locationsYAML);

    // Create instances of all the important stuff
    const preferences = new Preferences();
    const calculator = new Calculator(preferences);

    // Handle incoming requests
    onmessage = msg => {
        const { id, type, data } = msg.data;

        // console.debug(`Processing request #${id} — ${type} - ${JSON.stringify(data)}`);

        let response;

        switch (type) {
            case 'REACHABILITY_GEOJSON': {
                const { aircraftID, altitude } = data;
                const aircraftInstance = aircrafts.get(aircraftID);
                response = calculator.reachabilityGeoJSON(locations, aircraftInstance, altitude);
                break;
            }
            case 'LOCATION_LINES_GEOJSON': {
                const { aircraftID } = data;
                const aircraft = aircrafts.get(aircraftID);
                response = calculator.locationGeoJSON(locations, aircraft);
                break;
            }
            case 'CLOSEST_LOCATION_ID': {
                const { latitude, longitude } = data;
                response = locations.closest(latitude, longitude);
                break;
            }
            case 'LOCATION_DATA': {
                const { locationID, aircraftID } = data;
                const aircraft = aircrafts.get(aircraftID);
                const location = locations.get(locationID);
                response = serializeLocation(location, aircraft, calculator);
                break;
            }
            case 'AIRCRAFT_LIST': {
                response = Array.from(aircrafts.values()).map(serializeAircraft)
                break;
            }
            case 'LANDING_OPTIONS': {
                const { latitude, longitude, heading, altitude, aircraftID } = data;
                const aircraft = aircrafts.get(aircraftID);
                response = calculator.landingOptions(latitude, longitude, heading, altitude, aircraft, locations);
                break;
            }
            default:
                console.error('Received unknown worker request', type);
        }

        postMessage({
            id,
            response
        });
    }

    postMessage({ id: 'startup', response: { aircraftCount: aircrafts.size, locationCount: locations.keys().length } });
}

run()
    .then(() => console.info('Worker ready.'))
    .catch(e => console.error('Worker failed:', e));

function serializeLocation(location, aircraft, calculator) {
    return {
        id: location.id,
        name: location.name,

        coordinates: location.coordinates,
        length: location.length,
        reversible: location.reversible,

        bearing: location.bearing,
        reverseBearing: location.reversible ? location.reverseBearing : null,

        usage: location.usage,
        surface: location.surface,
        humanPresence: location.humanPresence,

        risk: calculator.assessRisk(location, aircraft),
        landingHeadroom: location.landingHeadroom(aircraft),
    };
}

function serializeAircraft(aircraft) {
    return {
        id: aircraft.id,
        name: aircraft.name,
        mtow: aircraft.mtow,
        takeoff: {
            groundRoll: aircraft.takeoff.groundRoll,
            totalDistance: aircraft.takeoff.totalDistance,
        },
        climb: {
            rate: aircraft.climb.rate
        },
        glide: {
            ratio: aircraft.glide.ratio,
            // TODO Use the bank angle from preferences!
            turnRadius: aircraft.glide.turnRadius(45)
        },
        landing: {
            groundRoll: aircraft.landing.groundRoll,
            totalDistance: aircraft.landing.totalDistance,
        },
    }
}