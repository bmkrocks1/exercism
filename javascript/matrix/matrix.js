//
// This is only a SKELETON file for the 'Matrix' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export class Matrix {
  constructor(str) {
    this._matrix = str
      .split('\n')
      .map(
        row => row
          .trim()
          .split(' ')
          .map(num => Number(num))
      );
  }

  get rows() {
    return this._matrix;
  }

  get columns() {
    return this._matrix.reduce((cols, row) => {
      row.forEach((num, index) => {
        cols[index] = cols[index] || [];
        cols[index].push(num);
      });
      return cols;
    }, []);
  }
}