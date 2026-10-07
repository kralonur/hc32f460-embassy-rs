#[doc = "Register `USBFS_SYCTLREG` reader"]
pub type R = crate::R<UsbfsSyctlregSpec>;
#[doc = "Register `USBFS_SYCTLREG` writer"]
pub type W = crate::W<UsbfsSyctlregSpec>;
#[doc = "Field `DFB` reader - desc DFB"]
pub type DfbR = crate::BitReader;
#[doc = "Field `DFB` writer - desc DFB"]
pub type DfbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFEN` reader - desc SOFEN"]
pub type SofenR = crate::BitReader;
#[doc = "Field `SOFEN` writer - desc SOFEN"]
pub type SofenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc DFB"]
    #[inline(always)]
    pub fn dfb(&self) -> DfbR {
        DfbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SOFEN"]
    #[inline(always)]
    pub fn sofen(&self) -> SofenR {
        SofenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc DFB"]
    #[inline(always)]
    pub fn dfb(&mut self) -> DfbW<'_, UsbfsSyctlregSpec> {
        DfbW::new(self, 0)
    }
    #[doc = "Bit 1 - desc SOFEN"]
    #[inline(always)]
    pub fn sofen(&mut self) -> SofenW<'_, UsbfsSyctlregSpec> {
        SofenW::new(self, 1)
    }
}
#[doc = "desc USBFS_SYCTLREG\n\nYou can [`read`](crate::Reg::read) this register and get [`usbfs_syctlreg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbfs_syctlreg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbfsSyctlregSpec;
impl crate::RegisterSpec for UsbfsSyctlregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbfs_syctlreg::R`](R) reader structure"]
impl crate::Readable for UsbfsSyctlregSpec {}
#[doc = "`write(|w| ..)` method takes [`usbfs_syctlreg::W`](W) writer structure"]
impl crate::Writable for UsbfsSyctlregSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBFS_SYCTLREG to value 0"]
impl crate::Resettable for UsbfsSyctlregSpec {}
