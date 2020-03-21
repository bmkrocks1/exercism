//
// This is only a SKELETON file for the 'Space Age' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

const EARTH_YEAR_IN_SECONDS = 31557600;

const ORBITAL_PERIOD_MAP = {
  mercury: 0.2408467,
  venus: 0.61519726,
  earth: 1,
  mars: 1.8808158,
  jupiter: 11.862615,
  saturn: 29.447498,
  uranus: 84.016846,
  neptune: 164.79132
};

export const age = (planet, seconds) => {
  const earthYears = seconds / (ORBITAL_PERIOD_MAP[planet] * EARTH_YEAR_IN_SECONDS);
  return Number(earthYears.toFixed(2));
};
