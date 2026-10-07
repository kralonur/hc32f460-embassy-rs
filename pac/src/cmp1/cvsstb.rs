#[doc = "Register `CVSSTB` reader"]
pub type R = crate::R<CvsstbSpec>;
#[doc = "Register `CVSSTB` writer"]
pub type W = crate::W<CvsstbSpec>;
#[doc = "Field `STB` reader - desc STB"]
pub type StbR = crate::FieldReader;
#[doc = "Field `STB` writer - desc STB"]
pub type StbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc STB"]
    #[inline(always)]
    pub fn stb(&self) -> StbR {
        StbR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc STB"]
    #[inline(always)]
    pub fn stb(&mut self) -> StbW<'_, CvsstbSpec> {
        StbW::new(self, 0)
    }
}
#[doc = "desc CVSSTB\n\nYou can [`read`](crate::Reg::read) this register and get [`cvsstb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvsstb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CvsstbSpec;
impl crate::RegisterSpec for CvsstbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cvsstb::R`](R) reader structure"]
impl crate::Readable for CvsstbSpec {}
#[doc = "`write(|w| ..)` method takes [`cvsstb::W`](W) writer structure"]
impl crate::Writable for CvsstbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CVSSTB to value 0x05"]
impl crate::Resettable for CvsstbSpec {
    const RESET_VALUE: u16 = 0x05;
}
