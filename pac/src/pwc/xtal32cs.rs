#[doc = "Register `XTAL32CS` reader"]
pub type R = crate::R<Xtal32csSpec>;
#[doc = "Register `XTAL32CS` writer"]
pub type W = crate::W<Xtal32csSpec>;
#[doc = "Field `CSDIS` reader - desc CSDIS"]
pub type CsdisR = crate::BitReader;
#[doc = "Field `CSDIS` writer - desc CSDIS"]
pub type CsdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - desc CSDIS"]
    #[inline(always)]
    pub fn csdis(&self) -> CsdisR {
        CsdisR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - desc CSDIS"]
    #[inline(always)]
    pub fn csdis(&mut self) -> CsdisW<'_, Xtal32csSpec> {
        CsdisW::new(self, 7)
    }
}
#[doc = "desc XTAL32CS\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal32cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal32cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xtal32csSpec;
impl crate::RegisterSpec for Xtal32csSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xtal32cs::R`](R) reader structure"]
impl crate::Readable for Xtal32csSpec {}
#[doc = "`write(|w| ..)` method takes [`xtal32cs::W`](W) writer structure"]
impl crate::Writable for Xtal32csSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTAL32CS to value 0x02"]
impl crate::Resettable for Xtal32csSpec {
    const RESET_VALUE: u8 = 0x02;
}
