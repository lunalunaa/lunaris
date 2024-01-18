// https://www.rpi4os.com/part1-bootstrapping/

// ***************************************
// SCTLR_EL1, System Control Register (EL1)
// Architecture Reference Manual Section D17.2.118
// ***************************************
#define SCTLR_RESERVED (3 << 28) | (3 << 22) | (1 << 20) | (1 << 11)
#define USER_MASK_ACCESS (1 << 9)
#define SCTLR_WFE_WFI_ENABLED (1 << 18) | (1 << 16)
#define SCTLR_VALUE_MMU_DISABLED                                               \
  (SCTLR_RESERVED | USER_MASK_ACCESS | SCTLR_WFE_WFI_ENABLED)

// ***************************************
// HCR_EL2, Hypervisor Configuration Register (EL2)
// Architecture Reference Manual Section D17.2.48
// ***************************************
#define HCR_RW (1 << 31)

// ***************************************
// SPSR_EL2, Saved Program Status Register (EL2)
// Architecture Reference Manual Section C5.2.19
// ***************************************
#define SPSR_MASK_ALL (11 << 6)
#define SPSR_EL1 (5 << 0)
#define SPSR_VALUE (SPSR_MASK_ALL | SPSR_EL1)
#include <iostream>
using namespace std;
int main() {
  cout << "HCR_RW = " << HCR_RW << endl;
  cout << "SPSR_VALUE = " << SPSR_VALUE << endl;
  cout << "SCTLR_VALUE_MMU_DISABLED = " << SCTLR_VALUE_MMU_DISABLED << endl;
}
