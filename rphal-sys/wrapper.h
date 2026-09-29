// Entry point for bindgen. Parsed as C++ because several HAL headers
// (DMA, REVPH, PowerDistribution, most simulation headers) include C++
// standard headers; everything exported is still the extern "C" API.
//
// Not included (need NI roboRIO headers or are C++-only internals):
//   hal/ChipObject.h, hal/UsageReporting.h, hal/roborio/*, hal/cpp/*,
//   hal/handles/*, hal/simulation/SimCallbackRegistry.h,
//   hal/simulation/SimDataValue.h

#include <hal/HAL.h>
#include <hal/DMA.h>
#include <hal/DutyCycle.h>
#include <hal/Extensions.h>
#include <hal/PowerDistribution.h>
#include <hal/REVPH.h>

#include <hal/simulation/AccelerometerData.h>
#include <hal/simulation/AddressableLEDData.h>
#include <hal/simulation/AnalogGyroData.h>
#include <hal/simulation/AnalogInData.h>
#include <hal/simulation/AnalogOutData.h>
#include <hal/simulation/AnalogTriggerData.h>
#include <hal/simulation/CTREPCMData.h>
#include <hal/simulation/CanData.h>
#include <hal/simulation/DIOData.h>
#include <hal/simulation/DigitalPWMData.h>
#include <hal/simulation/DriverStationData.h>
#include <hal/simulation/DutyCycleData.h>
#include <hal/simulation/EncoderData.h>
#include <hal/simulation/I2CData.h>
#include <hal/simulation/MockHooks.h>
#include <hal/simulation/NotifierData.h>
#include <hal/simulation/NotifyListener.h>
#include <hal/simulation/PWMData.h>
#include <hal/simulation/PowerDistributionData.h>
#include <hal/simulation/REVPHData.h>
#include <hal/simulation/RelayData.h>
#include <hal/simulation/Reset.h>
#include <hal/simulation/RoboRioData.h>
#include <hal/simulation/SPIAccelerometerData.h>
#include <hal/simulation/SPIData.h>
#include <hal/simulation/SimDeviceData.h>
