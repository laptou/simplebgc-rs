//! copied from spec pdf and then edited using
//! regex replace
#![allow(unused)]
pub(crate) const CMD_READ_PARAMS: u8 = 82;
pub(crate) const CMD_WRITE_PARAMS: u8 = 87;
pub(crate) const CMD_REALTIME_DATA: u8 = 68;
pub(crate) const CMD_BOARD_INFO: u8 = 86;
pub(crate) const CMD_CALIB_ACC: u8 = 65;
pub(crate) const CMD_CALIB_GYRO: u8 = 103;
pub(crate) const CMD_CALIB_EXT_GAIN: u8 = 71;
pub(crate) const CMD_USE_DEFAULTS: u8 = 70;
pub(crate) const CMD_CALIB_POLES: u8 = 80;
pub(crate) const CMD_RESET: u8 = 114;
pub(crate) const CMD_HELPER_DATA: u8 = 72;
pub(crate) const CMD_CALIB_OFFSET: u8 = 79;
pub(crate) const CMD_CALIB_BAT: u8 = 66;
pub(crate) const CMD_MOTORS_ON: u8 = 77;
pub(crate) const CMD_MOTORS_OFF: u8 = 109;
pub(crate) const CMD_CONTROL: u8 = 67;
pub(crate) const CMD_TRIGGER_PIN: u8 = 84;
pub(crate) const CMD_EXECUTE_MENU: u8 = 69;
pub(crate) const CMD_GET_ANGLES: u8 = 73;
pub(crate) const CMD_CONFIRM: u8 = 67;
pub(crate) const CMD_BOARD_INFO_3: u8 = 20;
pub(crate) const CMD_READ_PARAMS_3: u8 = 21;
pub(crate) const CMD_WRITE_PARAMS_3: u8 = 22;
pub(crate) const CMD_REALTIME_DATA_3: u8 = 23;
pub(crate) const CMD_REALTIME_DATA_4: u8 = 25;
pub(crate) const CMD_SELECT_IMU_3: u8 = 24;
pub(crate) const CMD_READ_PROFILE_NAMES: u8 = 28;
pub(crate) const CMD_WRITE_PROFILE_NAMES: u8 = 29;
pub(crate) const CMD_QUEUE_PARAMS_INFO_3: u8 = 30;
pub(crate) const CMD_SET_ADJ_VARS_VAL: u8 = 31;
pub(crate) const CMD_SAVE_PARAMS_3: u8 = 32;
pub(crate) const CMD_READ_PARAMS_EXT: u8 = 33;
pub(crate) const CMD_WRITE_PARAMS_EXT: u8 = 34;
pub(crate) const CMD_AUTO_PID: u8 = 35;
pub(crate) const CMD_SERVO_OUT: u8 = 36;
pub(crate) const CMD_I2C_WRITE_REG_BUF: u8 = 39;
pub(crate) const CMD_I2C_READ_REG_BUF: u8 = 40;
pub(crate) const CMD_WRITE_EXTERNAL_DATA: u8 = 41;
pub(crate) const CMD_READ_EXTERNAL_DATA: u8 = 42;
pub(crate) const CMD_READ_ADJ_VARS_CFG: u8 = 43;
pub(crate) const CMD_WRITE_ADJ_VARS_CFG: u8 = 44;
pub(crate) const CMD_API_VIRT_CH_CONTROL: u8 = 45;
pub(crate) const CMD_ADJ_VARS_STATE: u8 = 46;
pub(crate) const CMD_EEPROM_WRITE: u8 = 47;
pub(crate) const CMD_EEPROM_READ: u8 = 48;
pub(crate) const CMD_CALIB_INFO: u8 = 49;
pub(crate) const CMD_SIGN_MESSAGE: u8 = 50;
pub(crate) const CMD_BOOT_MODE_3: u8 = 51;
pub(crate) const CMD_SYSTEM_STATE: u8 = 52;
pub(crate) const CMD_READ_FILE: u8 = 53;
pub(crate) const CMD_WRITE_FILE: u8 = 54;
pub(crate) const CMD_FS_CLEAR_ALL: u8 = 55;
pub(crate) const CMD_AHRS_HELPER: u8 = 56;
pub(crate) const CMD_RUN_SCRIPT: u8 = 57;
pub(crate) const CMD_SCRIPT_DEBUG: u8 = 58;
pub(crate) const CMD_CALIB_MAG: u8 = 59;
pub(crate) const CMD_GET_ANGLES_EXT: u8 = 61;
pub(crate) const CMD_READ_PARAMS_EXT2: u8 = 62;
pub(crate) const CMD_WRITE_PARAMS_EXT2: u8 = 63;
pub(crate) const CMD_GET_ADJ_VARS_VAL: u8 = 64;
pub(crate) const CMD_CALIB_MOTOR_MAG_LINK: u8 = 74;
pub(crate) const CMD_GYRO_CORRECTION: u8 = 75;
pub(crate) const CMD_DATA_STREAM_INTERVAL: u8 = 85;
pub(crate) const CMD_REALTIME_DATA_CUSTOM: u8 = 88;
pub(crate) const CMD_BEEP_SOUND: u8 = 89;
pub(crate) const CMD_ENCODERS_CALIB_OFFSET_4: u8 = 26;
pub(crate) const CMD_ENCODERS_CALIB_FLD_OFFSET_4: u8 = 27;
pub(crate) const CMD_CONTROL_CONFIG: u8 = 90;
pub(crate) const CMD_CALIB_ORIENT_CORR: u8 = 91;
pub(crate) const CMD_COGGING_CALIB_INFO: u8 = 92;
pub(crate) const CMD_CALIB_COGGING: u8 = 93;
pub(crate) const CMD_CALIB_ACC_EXT_REF: u8 = 94;
pub(crate) const CMD_PROFILE_SET: u8 = 95;
pub(crate) const CMD_CAN_DEVICE_SCAN: u8 = 96;
pub(crate) const CMD_CAN_DRV_HARD_PARAMS: u8 = 97;
pub(crate) const CMD_CAN_DRV_STATE: u8 = 98;
pub(crate) const CMD_CAN_DRV_CALIBRATE: u8 = 99;
pub(crate) const CMD_READ_RC_INPUTS: u8 = 100;
pub(crate) const CMD_REALTIME_DATA_CAN_DRV: u8 = 101;
pub(crate) const CMD_EVENT: u8 = 102;
pub(crate) const CMD_READ_PARAMS_EXT3: u8 = 104;
pub(crate) const CMD_WRITE_PARAMS_EXT3: u8 = 105;
pub(crate) const CMD_EXT_IMU_DEBUG_INFO: u8 = 106;
pub(crate) const CMD_SET_DEVICE_ADDR: u8 = 107;
pub(crate) const CMD_AUTO_PID2: u8 = 108;
pub(crate) const CMD_EXT_IMU_CMD: u8 = 110;
pub(crate) const CMD_READ_STATE_VARS: u8 = 111;
pub(crate) const CMD_WRITE_STATE_VARS: u8 = 112;
pub(crate) const CMD_SERIAL_PROXY: u8 = 113;
pub(crate) const CMD_IMU_ADVANCED_CALIB: u8 = 115;
pub(crate) const CMD_API_VIRT_CH_HIGH_RES: u8 = 116;
pub(crate) const CMD_SET_DEBUG_PORT: u8 = 249;
pub(crate) const CMD_MAVLINK_INFO: u8 = 250;
pub(crate) const CMD_MAVLINK_DEBUG: u8 = 251;
pub(crate) const CMD_DEBUG_VARS_INFO_3: u8 = 253;
pub(crate) const CMD_DEBUG_VARS_3: u8 = 254;
pub(crate) const CMD_ERROR: u8 = 255;
