## The PCB
Color/shades options and combinations (for the green pcb):
* silkscreen (white)
* copper with no solder mask (gold or silver)
* copper with solder mask (light green)
* solder mask without copper under (dark green)
* no solder mask and no copper (FR4 collor)


| foreground \ background | white | gold/silver | dark green | light green | clear |
| -------------           |-------| ----------- | ---------- |-------------|-------|
| white                   | X     |             |            |             |       |
| gold/silver             |       |  X          |            |             |       |
| dark green              |       |             |       X    |             |       |
| light green             |       |             |            | X           |       |
| clear                   |       |             |            |             | X     |


## Bill Of Materials

### Astable Multivibrator
|Component  |Quantity |Link   |
|-----      |-----    |-----  |
|Ceramic Capacitor 0603 470nF   |2 |[digikey](https://www.digikey.se/en/products/detail/samsung-electro-mechanics/CL10B474KA8NNWC/3887742) |
|Ceramic Capacitor 0603 10nF   |1 |[digikey](https://www.digikey.se/en/products/detail/kemet/C0603C102M3RECAUTO/6826026) |
|Ceramic Capacitor 0603 100pF   |1 |[digikey](https://www.digikey.se/en/products/detail/american-technical-ceramics/600S101KT250XT/3908988) |
|Red LED 0603| 20| [digikey](https://www.digikey.se/en/products/detail/liteon/LTST-C191KRKT/386837) |
|Green LED 0603| 20| [digikey](https://www.digikey.se/en/mylists/list/9087QGF5WM) |
|Resistor 0603 470k| 2| [digikey](https://www.digikey.se/en/products/detail/yageo/RC0603JR-07470KL/726792) |
|Resistor 0603 470|40 |[digikey](https://www.digikey.se/en/products/detail/vishay-beyschlag-draloric-bc-components/MCT0603MD4700DP500/2092094) |
|Resistor 0603 1k 5%| 4 |[digikey](https://www.digikey.se/en/products/detail/walsin-technology-corporation/WR06X102-JTL/13241138) |
|Resistor 0603 10k 5%| 3 |[digikey](https://www.digikey.se/en/products/detail/te-connectivity-passive-product/CRG0603J10K/2389998) |
|Resistor 0603 100k 5%| 2 |[digikey](https://www.digikey.se/en/products/detail/yageo/RC0603FR-07100KL/726889) |
|Slide switch|3|[digikey](https://www.digikey.se/en/products/detail/c-k/PCM12SMTR/1640112?utm_medium=aggregator&utm_source=snapeda&utm_campaign=buynow) |
|Transistor SOT-23 BC846|2|[digikey](https://www.digikey.se/en/products/detail/nexperia-usa-inc/BC846B-235/1232263) |

### ESP32-C3 shenanigans
|Component  |Quantity |Link   |
|-----      |-----    |-----  |
|ESP32-C3-MINI-1-N4-A|1 |[digikey](https://www.digikey.se/en/products/detail/espressif-systems/ESP32-C3-MINI-1-N4-A/15817506) |

### Common
|Component  |Quantity |Link   |
|-----      |-----    |-----  |
|CR2032 battery holder - BK-912-TR|2        |[digikey](https://www.digikey.se/en/products/detail/mpd-memory-protection-devices/BK-912-TR/2077831?s=N4IgTCBcDaIEIGkC0BOAjGJBhAKkgcgCIgC6AvkA) |
|LDO 3.3V 600mA|1        |[digikey](https://www.digikey.se/en/products/detail/diodes-incorporated/AP7366-33W5-7/9867322) |
|MMBT3906-7-F PNP Transistor|2        |[digikey](https://www.digikey.se/en/products/detail/diodes-incorporated/MMBT3906-7-F/770797) |
|P-Channel MOSFET SSM3J56MFV,L3F|1        |[digikey](https://www.digikey.se/en/products/detail/toshiba-semiconductor-and-storage/SSM3J56MFV-L3F/4305160) |
|Transistor SOT-23 BC846|2|[digikey](https://www.digikey.se/en/products/detail/nexperia-usa-inc/BC846B-235/1232263) |
|USB-C|1        |[digikey](https://www.digikey.se/en/products/detail/gct/USB4105-GF-A/11198441) |
|Zener diode MMSZ5233B-AU_R1_000A1|1        |[digikey](https://www.digikey.se/en/products/detail/panjit-international-inc/MMSZ5233B-AU-R1-000A1/15796616) |



## TODO:
- ESP with neopixels
	- reset button
	- JTAG
	- UART
	- neopixels
	- temp. sensor
- test points
- connect everything on the PCB
- test shining lights through the PCB

