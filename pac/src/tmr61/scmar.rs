#[doc = "Register `SCMAR` reader"]
pub type R = crate::R<ScmarSpec>;
#[doc = "Register `SCMAR` writer"]
pub type W = crate::W<ScmarSpec>;
#[doc = "Field `SCMA` reader - desc SCMA"]
pub type ScmaR = crate::FieldReader<u16>;
#[doc = "Field `SCMA` writer - desc SCMA"]
pub type ScmaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - desc SCMA"]
    #[inline(always)]
    pub fn scma(&self) -> ScmaR {
        ScmaR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - desc SCMA"]
    #[inline(always)]
    pub fn scma(&mut self) -> ScmaW<'_, ScmarSpec> {
        ScmaW::new(self, 0)
    }
}
#[doc = "desc SCMAR\n\nYou can [`read`](crate::Reg::read) this register and get [`scmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmarSpec;
impl crate::RegisterSpec for ScmarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scmar::R`](R) reader structure"]
impl crate::Readable for ScmarSpec {}
#[doc = "`write(|w| ..)` method takes [`scmar::W`](W) writer structure"]
impl crate::Writable for ScmarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCMAR to value 0xffff"]
impl crate::Resettable for ScmarSpec {
    const RESET_VALUE: u32 = 0xffff;
}
