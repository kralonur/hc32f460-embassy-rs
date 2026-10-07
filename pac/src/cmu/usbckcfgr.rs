#[doc = "Register `USBCKCFGR` reader"]
pub type R = crate::R<UsbckcfgrSpec>;
#[doc = "Register `USBCKCFGR` writer"]
pub type W = crate::W<UsbckcfgrSpec>;
#[doc = "Field `USBCKS` reader - desc USBCKS"]
pub type UsbcksR = crate::FieldReader;
#[doc = "Field `USBCKS` writer - desc USBCKS"]
pub type UsbcksW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 4:7 - desc USBCKS"]
    #[inline(always)]
    pub fn usbcks(&self) -> UsbcksR {
        UsbcksR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 4:7 - desc USBCKS"]
    #[inline(always)]
    pub fn usbcks(&mut self) -> UsbcksW<'_, UsbckcfgrSpec> {
        UsbcksW::new(self, 4)
    }
}
#[doc = "desc USBCKCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`usbckcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbckcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbckcfgrSpec;
impl crate::RegisterSpec for UsbckcfgrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`usbckcfgr::R`](R) reader structure"]
impl crate::Readable for UsbckcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`usbckcfgr::W`](W) writer structure"]
impl crate::Writable for UsbckcfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBCKCFGR to value 0x40"]
impl crate::Resettable for UsbckcfgrSpec {
    const RESET_VALUE: u8 = 0x40;
}
