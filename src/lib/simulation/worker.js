import init, { Parser, Calculator, Preferences } from 'elsa';
import { dev } from '$app/env';

function parsePrefs(preferences) {
    return new Preferences(JSON.stringify(preferences))
}

async function run() {
    // Start fetching everything we need
    const elsaPromise = init(dev ? undefined : '/assets/elsa.wasm');
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
    const calculator = new Calculator();

    // Handle incoming requests
    onmessage = msg => {
        const { id, type, data } = msg.data;

        const logData = Object.assign({}, data);
        delete logData.preferences;
        console.debug(`Processing request #${id} — ${type} - ${JSON.stringify(logData)}`);

        let response;

        switch (type) {
            case 'REACHABILITY_GEOJSON': {
                let { preferences, aircraftID, altitude } = data;
                const aircraftInstance = aircrafts.get(aircraftID);
                response = calculator.reachabilityGeoJSON(parsePrefs(preferences), locations, aircraftInstance, altitude);
                break;
            }
            case 'LOCATION_LINES_GEOJSON': {
                let { preferences, aircraftID } = data;
                const aircraft = aircrafts.get(aircraftID);
                response = calculator.locationGeoJSON(parsePrefs(preferences), locations, aircraft);
                break;
            }
            case 'LOCATION_HITBOXES': {
                const { distance } = data;
                response = calculator.locationHitboxes(locations, distance);
                break;
            }
            case 'CLOSEST_LOCATION_ID': {
                const { latitude, longitude } = data;
                response = locations.closest(latitude, longitude);
                break;
            }
            case 'LOCATION_DATA': {
                let { preferences, locationID, aircraftID } = data;
                const aircraft = aircrafts.get(aircraftID);
                const location = locations.get(locationID);
                response = serializeLocation(location, aircraft, calculator, parsePrefs(preferences));
                break;
            }
            case 'AIRCRAFT_LIST': {
                response = Array.from(aircrafts.values()).map(serializeAircraft)
                break;
            }
            case 'AIRCRAFT': {
                const { aircraftID } = data;
                response = serializeAircraft(aircrafts.get(aircraftID));
                break;
            }
            case 'LANDING_OPTIONS': {
                let { preferences, latitude, longitude, heading, altitude, aircraftID } = data;
                const aircraft = aircrafts.get(aircraftID);
                response = calculator.landingOptions(parsePrefs(preferences), latitude, longitude, heading, altitude, aircraft, locations);
                break;
            }
            case 'TAKEOFF_PROFILE': {
                const { aircraftID, distance } = data;
                const aircraft = aircrafts.get(aircraftID);
                response = calculator.takeoffProfile(aircraft, distance);
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

function serializeLocation(location, aircraft, calculator, preferences) {
    if (!location) return null;

    return {
        id: location.id,
        name: location.name,

        coordinates: location.coordinates,
        length: location.length,
        elevation: location.elevation,
        reversible: location.reversible,

        bearing: location.bearing,
        reverseBearing: location.reversible ? location.reverseBearing : null,

        usage: location.usage,
        surface: location.surface,
        humanPresence: location.humanPresence,

        risk: calculator.assessRisk(preferences, location, aircraft).toJSON(),
        landingHeadroom: location.landingHeadroom(aircraft),

        surveyDate: location.surveyDate,
        remarks: location.remarks
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
            turnRadius: aircraft.glide.turnRadius(45)
        },
        landing: {
            groundRoll: aircraft.landing.groundRoll,
            totalDistance: aircraft.landing.totalDistance,
        },
    }
}