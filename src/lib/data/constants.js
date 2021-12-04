import { degreesToRadians } from '@turf/helpers';

export const riskCategories = ['safe', 'risky', 'unsafe'];
export const riskColors = {
    safe: '#388E3C',
    risky: '#FFC107',
    unsafe: '#E64A19'
};

export const guideLocation = '786e1b618b69f695'

export const bankAngles = [45, 60];
export const riskyLandingHeadrooms = [0, -0.05, -0.1, -0.15];
export const unsafeLandingHeadrooms = [0, -0.05, -0.1, -0.15, -0.2, -0.25];
export const defaultPreferences = {
    bank: degreesToRadians(bankAngles[0]),
    epsilon: 0.1,

    riskyLandingHeadroom: riskyLandingHeadrooms[1],
    unsafeLandingHeadroom: unsafeLandingHeadrooms[3],

    eventLocationClassification: riskCategories[1],
    denselyCrowdedClassification: riskCategories[2],
};
