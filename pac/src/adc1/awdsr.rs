#[doc = "Register `AWDSR` reader"]
pub type R = crate::R<AwdsrSpec>;
#[doc = "Register `AWDSR` writer"]
pub type W = crate::W<AwdsrSpec>;
#[doc = "Field `AWDF` reader - desc AWDF"]
pub type AwdfR = crate::FieldReader<u32>;
#[doc = "Field `AWDF` writer - desc AWDF"]
pub type AwdfW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - desc AWDF"]
    #[inline(always)]
    pub fn awdf(&self) -> AwdfR {
        AwdfR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - desc AWDF"]
    #[inline(always)]
    pub fn awdf(&mut self) -> AwdfW<'_, AwdsrSpec> {
        AwdfW::new(self, 0)
    }
}
#[doc = "desc AWDSR\n\nYou can [`read`](crate::Reg::read) this register and get [`awdsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awdsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AwdsrSpec;
impl crate::RegisterSpec for AwdsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awdsr::R`](R) reader structure"]
impl crate::Readable for AwdsrSpec {}
#[doc = "`write(|w| ..)` method takes [`awdsr::W`](W) writer structure"]
impl crate::Writable for AwdsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWDSR to value 0"]
impl crate::Resettable for AwdsrSpec {}
