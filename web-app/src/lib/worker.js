import init, { do_stuff, Foo } from 'gaelic';
import { dev } from '$app/env';

async function run() {
    // Start fetching everything we need
    const elsaPromise = init(dev ? undefined : '/elsa.wasm');
    const aircraftsPromise = fetch('/data/aircrafts.yml').then(res => res.text());
    const locationsPromise = fetch('/data/locations.yml').then(res => res.text());
    const dataPromise = Promise.all([aircraftsPromise, locationsPromise]);

    // Wait for all data to be fetched
    const [aircrafts, locations] = await dataPromise;
    await elsaPromise;

    // Start listening for stuff :)
    console.log('Hello world from worker!');
    postMessage('The answer is 42!');

    // Run some calculations
    console.time('calculate');
    do_stuff();
    console.timeEnd('calculate');

    const foo = new Foo(42);
    console.log('foo', foo);
    console.log('answer', foo.answer());
}

run()
    .then(() => console.info('Worker exited.'))
    .catch(e => console.error('Worker failed:', e));
