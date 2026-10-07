#[doc = "Register `MIN` reader"]
pub type R = crate::R<MinSpec>;
#[doc = "Register `MIN` writer"]
pub type W = crate::W<MinSpec>;
#[doc = "Field `MINU` reader - desc MINU"]
pub type MinuR = crate::FieldReader;
#[doc = "Field `MINU` writer - desc MINU"]
pub type MinuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MIND` reader - desc MIND"]
pub type MindR = crate::FieldReader;
#[doc = "Field `MIND` writer - desc MIND"]
pub type MindW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - desc MINU"]
    #[inline(always)]
    pub fn minu(&self) -> MinuR {
        MinuR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - desc MIND"]
    #[inline(always)]
    pub fn mind(&self) -> MindR {
        MindR::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc MINU"]
    #[inline(always)]
    pub fn minu(&mut self) -> MinuW<'_, MinSpec> {
        MinuW::new(self, 0)
    }
    #[doc = "Bits 4:6 - desc MIND"]
    #[inline(always)]
    pub fn mind(&mut self) -> MindW<'_, MinSpec> {
        MindW::new(self, 4)
    }
}
#[doc = "desc MIN\n\nYou can [`read`](crate::Reg::read) this register and get [`min::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`min::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MinSpec;
impl crate::RegisterSpec for MinSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`min::R`](R) reader structure"]
impl crate::Readable for MinSpec {}
#[doc = "`write(|w| ..)` method takes [`min::W`](W) writer structure"]
impl crate::Writable for MinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIN to value 0"]
impl crate::Resettable for MinSpec {}
