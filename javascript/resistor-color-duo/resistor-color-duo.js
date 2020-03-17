//
// This is only a SKELETON file for the 'Resistor Color Duo' exercise. It's been provided as a
// convenience to get you started writing code faster.
//
import { colorCode, COLORS } from '../resistor-color/resistor-color';

export const decodedValue = ([color1, color2]) =>
  colorCode(color1) * 10 + colorCode(color2);
