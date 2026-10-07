#[doc = "Register `HNPTXFSIZ` reader"]
pub type R = crate::R<HnptxfsizSpec>;
#[doc = "Register `HNPTXFSIZ` writer"]
pub type W = crate::W<HnptxfsizSpec>;
#[doc = "Field `NPTXFSA` reader - desc NPTXFSA"]
pub type NptxfsaR = crate::FieldReader<u16>;
#[doc = "Field `NPTXFSA` writer - desc NPTXFSA"]
pub type NptxfsaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NPTXFD` reader - desc NPTXFD"]
pub type NptxfdR = crate::FieldReader<u16>;
#[doc = "Field `NPTXFD` writer - desc NPTXFD"]
pub type NptxfdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc NPTXFSA"]
    #[inline(always)]
    pub fn nptxfsa(&self) -> NptxfsaR {
        NptxfsaR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - desc NPTXFD"]
    #[inline(always)]
    pub fn nptxfd(&self) -> NptxfdR {
        NptxfdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc NPTXFSA"]
    #[inline(always)]
    pub fn nptxfsa(&mut self) -> NptxfsaW<'_, HnptxfsizSpec> {
        NptxfsaW::new(self, 0)
    }
    #[doc = "Bits 16:31 - desc NPTXFD"]
    #[inline(always)]
    pub fn nptxfd(&mut self) -> NptxfdW<'_, HnptxfsizSpec> {
        NptxfdW::new(self, 16)
    }
}
#[doc = "desc HNPTXFSIZ\n\nYou can [`read`](crate::Reg::read) this register and get [`hnptxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hnptxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HnptxfsizSpec;
impl crate::RegisterSpec for HnptxfsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hnptxfsiz::R`](R) reader structure"]
impl crate::Readable for HnptxfsizSpec {}
#[doc = "`write(|w| ..)` method takes [`hnptxfsiz::W`](W) writer structure"]
impl crate::Writable for HnptxfsizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HNPTXFSIZ to value 0x0200_0140"]
impl crate::Resettable for HnptxfsizSpec {
    const RESET_VALUE: u32 = 0x0200_0140;
}
