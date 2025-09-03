// Relevant registers:
// - CTRL_REG1_A (20h): Datarate must be non-zero to enable accelerometer
// - STATUS_REG_A (27h): ZYXDA bit shows if new data is available
// - OUT_X_L_A (28h), OUT_X_H_A (29h): X axis low and high bytes
// - OUT_Y_L_A (2Ah), OUT_Y_H_A (2Bh): Y axis low and high bytes
// - OUT_Z_L_A (2Ch), OUT_Z_H_A (2Dh): Z axis low and high bytes

// 1. Create an enum with (some of) the register addresses

// 2. Create a device struct

// 3. Implement the constructor and the register read and write functions

// 4. Implement a function that sets up the accelerometer so it starts running

// 5. Implement a function to read the X,Y,Z data if there's any available
