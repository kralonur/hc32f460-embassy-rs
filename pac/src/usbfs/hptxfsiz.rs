#[doc = "Register `HPTXFSIZ` reader"]
pub type R = crate::R<HptxfsizSpec>;
#[doc = "Register `HPTXFSIZ` writer"]
pub type W = crate::W<HptxfsizSpec>;
#[doc = "Field `PTXSA` reader - desc PTXSA"]
pub type PtxsaR = crate::FieldReader<u16>;
#[doc = "Field `PTXSA` writer - desc PTXSA"]
pub type PtxsaW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PTXFD` reader - desc PTXFD"]
pub type PtxfdR = crate::FieldReader<u16>;
#[doc = "Field `PTXFD` writer - desc PTXFD"]
pub type PtxfdW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:11 - desc PTXSA"]
    #[inline(always)]
    pub fn ptxsa(&self) -> PtxsaR {
        PtxsaR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:26 - desc PTXFD"]
    #[inline(always)]
    pub fn ptxfd(&self) -> PtxfdR {
        PtxfdR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - desc PTXSA"]
    #[inline(always)]
    pub fn ptxsa(&mut self) -> PtxsaW<'_, HptxfsizSpec> {
        PtxsaW::new(self, 0)
    }
    #[doc = "Bits 16:26 - desc PTXFD"]
    #[inline(always)]
    pub fn ptxfd(&mut self) -> PtxfdW<'_, HptxfsizSpec> {
        PtxfdW::new(self, 16)
    }
}
#[doc = "desc HPTXFSIZ\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HptxfsizSpec;
impl crate::RegisterSpec for HptxfsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hptxfsiz::R`](R) reader structure"]
impl crate::Readable for HptxfsizSpec {}
#[doc = "`write(|w| ..)` method takes [`hptxfsiz::W`](W) writer structure"]
impl crate::Writable for HptxfsizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HPTXFSIZ to value 0x0140_0280"]
impl crate::Resettable for HptxfsizSpec {
    const RESET_VALUE: u32 = 0x0140_0280;
}
