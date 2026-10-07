#[doc = "Register `DTDAR` reader"]
pub type R = crate::R<DtdarSpec>;
#[doc = "Register `DTDAR` writer"]
pub type W = crate::W<DtdarSpec>;
#[doc = "Field `DTDA` reader - desc DTDA"]
pub type DtdaR = crate::FieldReader<u16>;
#[doc = "Field `DTDA` writer - desc DTDA"]
pub type DtdaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc DTDA"]
    #[inline(always)]
    pub fn dtda(&self) -> DtdaR {
        DtdaR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc DTDA"]
    #[inline(always)]
    pub fn dtda(&mut self) -> DtdaW<'_, DtdarSpec> {
        DtdaW::new(self, 0)
    }
}
#[doc = "desc DTDAR\n\nYou can [`read`](crate::Reg::read) this register and get [`dtdar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtdar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtdarSpec;
impl crate::RegisterSpec for DtdarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtdar::R`](R) reader structure"]
impl crate::Readable for DtdarSpec {}
#[doc = "`write(|w| ..)` method takes [`dtdar::W`](W) writer structure"]
impl crate::Writable for DtdarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTDAR to value 0xffff"]
impl crate::Resettable for DtdarSpec {
    const RESET_VALUE: u32 = 0xffff;
}
