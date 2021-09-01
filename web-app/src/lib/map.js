import criticalArea from '$lib/data/criticalArea.json';
import bbox from '@turf/bbox';

const criticalAreaBoundingBox = bbox(criticalArea);
export const FLY_SPEED = 1.0;

export function overviewCamera(map) {
    return map.cameraForBounds(criticalAreaBoundingBox);
}

export function firstNonBackgroundLayer(map) {
    const layers = map.getStyle().layers;

    // Find the index of the first raster layer in the map style
    let firstNonBackgroundLayer;

    for (let i = 0; i < layers.length; i++) {
        if (layers[i].type === 'symbol') {
            firstNonBackgroundLayer = layers[i].id;
            break;
        }
    }

    return firstNonBackgroundLayer;
}
