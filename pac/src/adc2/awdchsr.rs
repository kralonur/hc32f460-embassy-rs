#[doc = "Register `AWDCHSR` reader"]
pub type R = crate::R<AwdchsrSpec>;
#[doc = "Register `AWDCHSR` writer"]
pub type W = crate::W<AwdchsrSpec>;
#[doc = "Field `AWDCH` reader - desc AWDCH"]
pub type AwdchR = crate::FieldReader<u16>;
#[doc = "Field `AWDCH` writer - desc AWDCH"]
pub type AwdchW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - desc AWDCH"]
    #[inline(always)]
    pub fn awdch(&self) -> AwdchR {
        AwdchR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - desc AWDCH"]
    #[inline(always)]
    pub fn awdch(&mut self) -> AwdchW<'_, AwdchsrSpec> {
        AwdchW::new(self, 0)
    }
}
#[doc = "desc AWDCHSR\n\nYou can [`read`](crate::Reg::read) this register and get [`awdchsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awdchsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwdchsrSpec;
impl crate::RegisterSpec for AwdchsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awdchsr::R`](R) reader structure"]
impl crate::Readable for AwdchsrSpec {}
#[doc = "`write(|w| ..)` method takes [`awdchsr::W`](W) writer structure"]
impl crate::Writable for AwdchsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWDCHSR to value 0"]
impl crate::Resettable for AwdchsrSpec {}
