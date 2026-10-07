#[doc = "Register `MMF_REMPRT` reader"]
pub type R = crate::R<MmfRemprtSpec>;
#[doc = "Register `MMF_REMPRT` writer"]
pub type W = crate::W<MmfRemprtSpec>;
#[doc = "Field `REMPRT` reader - desc REMPRT"]
pub type RemprtR = crate::FieldReader<u16>;
#[doc = "Field `REMPRT` writer - desc REMPRT"]
pub type RemprtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc REMPRT"]
    #[inline(always)]
    pub fn remprt(&self) -> RemprtR {
        RemprtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc REMPRT"]
    #[inline(always)]
    pub fn remprt(&mut self) -> RemprtW<'_, MmfRemprtSpec> {
        RemprtW::new(self, 0)
    }
}
#[doc = "desc MMF_REMPRT\n\nYou can [`read`](crate::Reg::read) this register and get [`mmf_remprt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmf_remprt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmfRemprtSpec;
impl crate::RegisterSpec for MmfRemprtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmf_remprt::R`](R) reader structure"]
impl crate::Readable for MmfRemprtSpec {}
#[doc = "`write(|w| ..)` method takes [`mmf_remprt::W`](W) writer structure"]
impl crate::Writable for MmfRemprtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMF_REMPRT to value 0"]
impl crate::Resettable for MmfRemprtSpec {}
