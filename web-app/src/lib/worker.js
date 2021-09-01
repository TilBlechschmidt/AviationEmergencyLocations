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
                const { aircraft, altitude } = data;
                const aircraftInstance = aircrafts.get(aircraft);
                const geoJSON = calculator.reachabilityGeoJSON(locations, aircraftInstance, altitude);
                response = geoJSON;
                break;
            }
            case 'LOCATION_GEOJSON': {
                const { aircraft } = data;
                const aircraftInstance = aircrafts.get(aircraft);
                const geoJSON = calculator.locationGeoJSON(locations, aircraftInstance);
                response = geoJSON;
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
