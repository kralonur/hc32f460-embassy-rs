#[doc = "Register `UVR` reader"]
pub type R = crate::R<UvrSpec>;
#[doc = "Register `UVR` writer"]
pub type W = crate::W<UvrSpec>;
#[doc = "Field `UVR` reader - desc UVR"]
pub type UvrR = crate::FieldReader<u16>;
#[doc = "Field `UVR` writer - desc UVR"]
pub type UvrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc UVR"]
    #[inline(always)]
    pub fn uvr(&self) -> UvrR {
        UvrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc UVR"]
    #[inline(always)]
    pub fn uvr(&mut self) -> UvrW<'_, UvrSpec> {
        UvrW::new(self, 0)
    }
}
#[doc = "desc UVR\n\nYou can [`read`](crate::Reg::read) this register and get [`uvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UvrSpec;
impl crate::RegisterSpec for UvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uvr::R`](R) reader structure"]
impl crate::Readable for UvrSpec {}
#[doc = "`write(|w| ..)` method takes [`uvr::W`](W) writer structure"]
impl crate::Writable for UvrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UVR to value 0"]
impl crate::Resettable for UvrSpec {}
