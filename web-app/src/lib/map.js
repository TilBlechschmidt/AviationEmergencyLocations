import union from '@turf/union';
import difference from '@turf/difference';
import criticalArea from '$lib/data/criticalArea.json';
import bbox from '@turf/bbox';
import { featureCollection, polygon } from '@turf/helpers';
import transformRotate from '@turf/transform-rotate';
import rhumbDestination from '@turf/rhumb-destination';

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
        // console.log(layers[i]);
        if (layers[i].type === 'symbol') {
            firstNonBackgroundLayer = layers[i].id;
            break;
        }
    }

    return firstNonBackgroundLayer;
}

const riskColors = {
    safe: '#388E3C',
    risky: '#FFC107',
    unsafe: '#E64A19'
};



function radToDeg(rad) {
    return rad * (180.0 / Math.PI);
}

function rangePolygon(location, aircraft, altitude) {
    const offset = aircraft.derivedPerformance.glide.turnRadius;
    const range = aircraft.range;
    const approachEnd = location.geojson.geometry.coordinates[0];
    const polygonCenter = rhumbDestination(approachEnd, offset / 1000, 180);

    const requiredDistance =
        aircraft.derivedPerformance.landing.distanceOnSurface[location.surface];
    const inset = Math.max(0, location.length - requiredDistance);

    // 1. Add the approach end half
    let points = range.slice(0, 18).map(([rad, slope, offset]) => {
        // mx + b = y
        const distance = slope * altitude + offset;
        const angle = radToDeg(rad);
        const point = rhumbDestination(polygonCenter, distance / 1000, angle);
        return point.geometry.coordinates;
    });

    // 2. If not reversible, add the back half just like normal
    if (!location.reversible) {
        const furthestLandingPoint = rhumbDestination(polygonCenter, inset / 1000, 0);

        let backPoints = range.slice(18).map(([rad, slope, offset]) => {
            const distance = slope * altitude + offset;
            const angle = radToDeg(rad);
            const point = rhumbDestination(furthestLandingPoint, distance / 1000, angle);
            return point.geometry.coordinates;
        });

        points = points.concat(backPoints);
    }

    // 3. If reversible, add the front mirror to the back
    else {
        const reversedPolygonCenter = rhumbDestination(
            polygonCenter,
            (offset * 2 + location.length) / 1000,
            0
        );

        let backPoints = range.slice(0, 18).map(([rad, slope, offset]) => {
            const distance = slope * altitude + offset;
            const angle = radToDeg(rad) + 180;
            const point = rhumbDestination(reversedPolygonCenter, distance / 1000, angle);
            return point.geometry.coordinates;
        });

        points = points.concat(backPoints);
    }

    // Add the first point as the last one to close the polygon
    points.push(points[0]);

    // Rotate the polygon into place
    let rangeProfile = polygon([points], { color: '#0000FF' });
    return transformRotate(rangeProfile, location.bearing, { pivot: approachEnd });
}

export function assessLocationRisk(location, aircraft) {
    if (location.surface == 'Water') return 'unsafe';

    const landingHeadroomRatio = location.landingHeadroomRatios[aircraft.id];
    if (landingHeadroomRatio < -0.25) return 'unsafe';
    if (landingHeadroomRatio < -0.15) return 'risky';

    if (location.humanPresence == 'Dense') return 'risky';
    if (location.humanPresence == 'EventOnly') return 'risky';

    return 'safe';
}

export function generateRangeGeoJSON(locations, aircraft, altitude) {
    // Find all range polygons per risk category
    const rangesByRisk = locations.reduce((acc, loc) => {
        const risk = assessLocationRisk(loc, aircraft);
        const range = rangePolygon(loc, aircraft, altitude);

        if (!acc.hasOwnProperty(risk)) acc[risk] = [range];
        else acc[risk].push(range);

        return acc;
    }, {});

    // Unify each risk category into one MultiPolygon
    Object.keys(rangesByRisk).forEach((risk) => {
        rangesByRisk[risk] = rangesByRisk[risk].reduce((previous, current) =>
            union(previous, current)
        );
        rangesByRisk[risk].properties.color = riskColors[risk];
    });

    if (rangesByRisk.hasOwnProperty('unsafe')) {
        if (rangesByRisk.hasOwnProperty('risky')) {
            rangesByRisk.unsafe = difference(rangesByRisk.unsafe, rangesByRisk.risky);
        }
        if (rangesByRisk.hasOwnProperty('safe')) {
            rangesByRisk.unsafe = difference(rangesByRisk.unsafe, rangesByRisk.safe);
        }

        if (rangesByRisk.unsafe == null) {
            delete rangesByRisk.unsafe;
        }
    }

    if (rangesByRisk.hasOwnProperty('risky') && rangesByRisk.hasOwnProperty('safe')) {
        rangesByRisk.risky = difference(rangesByRisk.risky, rangesByRisk.safe);

        if (rangesByRisk.risky == null) {
            delete rangesByRisk.risky;
        }
    }

    return featureCollection(Object.values(rangesByRisk));
}

export function generateLocationLineGeoJSON(locations, aircraft) {
    const lines = locations
        .filter((l) => l.usage != 'Aeronautical')
        .map((location) => {
            const risk = assessLocationRisk(location, aircraft);
            const line = location.geojson;
            line.properties.color = riskColors[risk];
            return line;
        });

    return featureCollection(lines);
}
