#[doc = "Register `DTUAR` reader"]
pub type R = crate::R<DtuarSpec>;
#[doc = "Register `DTUAR` writer"]
pub type W = crate::W<DtuarSpec>;
#[doc = "Field `DTUA` reader - desc DTUA"]
pub type DtuaR = crate::FieldReader<u16>;
#[doc = "Field `DTUA` writer - desc DTUA"]
pub type DtuaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc DTUA"]
    #[inline(always)]
    pub fn dtua(&self) -> DtuaR {
        DtuaR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc DTUA"]
    #[inline(always)]
    pub fn dtua(&mut self) -> DtuaW<'_, DtuarSpec> {
        DtuaW::new(self, 0)
    }
}
#[doc = "desc DTUAR\n\nYou can [`read`](crate::Reg::read) this register and get [`dtuar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtuar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtuarSpec;
impl crate::RegisterSpec for DtuarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtuar::R`](R) reader structure"]
impl crate::Readable for DtuarSpec {}
#[doc = "`write(|w| ..)` method takes [`dtuar::W`](W) writer structure"]
impl crate::Writable for DtuarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTUAR to value 0xffff"]
impl crate::Resettable for DtuarSpec {
    const RESET_VALUE: u32 = 0xffff;
}
